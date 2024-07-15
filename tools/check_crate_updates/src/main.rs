//! This tool loops over each manifest for the images and for each Bevy crate, checks the support
//! table in its readme to see if it can be updated, if so it updates the manifest.

use crate::manifest::Manifest;
use anyhow::{anyhow, Context};
use cached::proc_macro::cached;
use std::{env, fs, path, str::FromStr};
use table_extract::Table;
use ureq::{Agent, AgentBuilder};

mod manifest;

const EXCLUDE_CRATES: &[&str] = &["bevy", "rand", "rand_chacha", "wasm-bindgen"];

fn main() -> anyhow::Result<()> {
    let Some(manifest_dir) = env::args().nth(1) else {
        return Err(anyhow!("A directory must be passed as an argument"));
    };

    let agent = AgentBuilder::new()
        // Crates.io asks that you add a user agent with some way of contacting you
        // https://crates.io/data-access#api
        .user_agent("github.com/LiamGallagher737/learnbevy")
        .build();

    println!("Searching {manifest_dir}");

    let manifest_paths = fs::read_dir(&manifest_dir)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.to_string_lossy().ends_with(".Cargo.toml"));

    for path in manifest_paths {
        if let Err(e) = handle_manifest(&path, agent.clone()) {
            eprintln!("[ERROR] Error handling {path:?}: {e}");
        };
    }

    println!("Complete");

    Ok(())
}

fn handle_manifest(path: &path::PathBuf, agent: Agent) -> anyhow::Result<()> {
    let manifest_str = fs::read_to_string(path)?;
    let mut manifest = Manifest::from_str(&manifest_str)
        .map_err(|e| anyhow!("Failed to parse manifest at {path:?}\n{e}"))?;

    let bevy_version = manifest
        .get_dependency("bevy")
        .ok_or_else(|| anyhow!("Manifest does not contain Bevy"))?
        .get_version()
        .ok_or_else(|| anyhow!("Invalid Bevy version"))?
        .to_owned();

    let newest_versions = manifest
        .get_dependency_names()
        .unwrap() // we know bevy exists so it can't be empty
        .filter(|name| !EXCLUDE_CRATES.contains(name))
        .map(|name| fetch_crate(name, agent.clone()))
        .inspect(|res| {
            if let Err(e) = res {
                eprintln!("Error getting crate: {e}");
            }
        })
        .filter_map(|res| res.ok())
        .map(|c| {
            (
                c.data.name.clone(),
                get_newest_version(c, &bevy_version, agent.clone()),
            )
        })
        .filter_map(|(name, version)| version.map(|v| (name, v)).ok())
        .collect::<Vec<_>>();

    for (name, version) in newest_versions {
        if !manifest
            .get_dependency_mut(&name)
            .unwrap() // name is a result from dep list so it must exist
            .set_version(&version)
        {
            eprintln!("[WARNING] Failed to set value of {name} to {version}");
        }
    }

    Ok(())
}

fn get_newest_version(
    c: CrateResponse,
    bevy_version: &str,
    agent: Agent,
) -> anyhow::Result<String> {
    let readme = fetch_readme(&c, agent.clone()).with_context(|| "Failed to get readme")?;
    let table = find_support_table(&readme).with_context(|| "Failed to find support table")?;

    // currently assuming the bevy column is first
    let mut matching = Vec::new();
    for row in table.iter().map(|r| r.as_slice()) {
        let bevy = extract_version_from_cell(&row[0]);
        let others = extract_versions_from_cell(&row[1]);
        for other in others {
            if bevy.starts_with(bevy_version) {
                matching.push((bevy.clone(), other));
            }
        }
    }

    if matching.is_empty() {
        return Err(anyhow!("{} has no matches for {bevy_version}", c.data.name));
    }

    let newest = matching
        .iter()
        .map(|(_, other)| other.parse::<semver::VersionReq>())
        .inspect(|res| {
            if let Err(e) = res {
                eprintln!("[WARNING] Failed to parse: {e}");
            }
        })
        .filter_map(Result::ok)
        .map(|semver| {
            c.versions
                .iter()
                .map(|v| v.version.parse::<semver::Version>().unwrap())
                .filter(|v| semver.matches(v))
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    println!(
        "[INFO] The most recent version for {} compatible with Bevy {bevy_version} is {newest}",
        c.data.name
    );

    Ok(format!("={newest}"))
}

#[cached(
    result = true,
    ty = "cached::SizedCache<String, CrateResponse>",
    create = "{ cached::SizedCache::with_size(20) }",
    convert = r#"{ name.to_owned() }"#
)]
fn fetch_crate(name: &str, agent: Agent) -> anyhow::Result<CrateResponse> {
    agent
        .get(&format!("https://crates.io/api/v1/crates/{name}"))
        .call()
        .map_err(|e| anyhow!("Failed to fetch crate data for {name:?}\n{e}"))?
        .into_json::<CrateResponse>()
        .map_err(|e| anyhow!("Failed to parse crate data for {name:?}\n{e}"))
}

#[cached(
    result = true,
    ty = "cached::SizedCache<String, String>",
    create = "{ cached::SizedCache::with_size(20) }",
    convert = r#"{ c.data.name.clone() }"#
)]
fn fetch_readme(c: &CrateResponse, agent: Agent) -> anyhow::Result<String> {
    let path = &c.versions[0].readme_path; // index 0 is latest
    agent
        .get(&format!("https://crates.io{path}"))
        .call()
        .map_err(|e| anyhow!("Failed to fetch readme\n{e}"))?
        .into_string()
        .map_err(|e| anyhow!("Failed to read readme\n{e}"))
}

fn find_support_table(readme: &str) -> anyhow::Result<Table> {
    const BEVY_HEADER_PHRASES: &[&str] = &["bevy", "Bevy", "Bevy version"];
    let find = |phrase: &str| Table::find_by_headers(readme, &[phrase]);
    for phrase in BEVY_HEADER_PHRASES {
        if let Some(table) = find(phrase) {
            return Ok(table);
        }
    }
    Err(anyhow!("Failed to find support table in readme"))
}

fn extract_version_from_cell(input: &str) -> String {
    input
        .chars()
        .filter(|&c| c.is_ascii_digit() || c == '.')
        .collect()
}

fn extract_versions_from_cell(input: &str) -> Vec<String> {
    input
        .split([' ', ',', '-'])
        .flat_map(|s| s.split(".."))
        .map(extract_version_from_cell)
        .filter(|s| !s.is_empty())
        .collect()
}

#[derive(serde::Deserialize, Clone)]
struct CrateResponse {
    #[serde(rename = "crate")]
    data: CrateDataResponse,
    versions: Vec<CrateVersionResponse>,
}

#[derive(serde::Deserialize, Clone)]
struct CrateDataResponse {
    name: String,
}

#[derive(serde::Deserialize, Clone)]
struct CrateVersionResponse {
    #[serde(rename = "num")]
    version: String,
    readme_path: String,
}
