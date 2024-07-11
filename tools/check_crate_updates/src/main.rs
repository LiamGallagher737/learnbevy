use anyhow::anyhow;
use cargo_toml::Manifest;
use std::{env, fs};
use table_extract::Table;
use ureq::{Agent, AgentBuilder};

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
        let manifest = Manifest::from_path(&path)
            .map_err(|e| anyhow!("Failed to parse manifest at {path:?}\n{e}"))?;

        let Some(bevy_version) = manifest.dependencies["bevy"]
            .detail()
            .and_then(|d| d.version.clone())
        else {
            eprintln!("Failed to get Bevy version of {path:?}");
            continue;
        };

        let crates = manifest
            .dependencies
            .iter()
            .map(|(name, _)| name)
            .filter(|name| !EXCLUDE_CRATES.contains(&name.as_str()))
            .map(|name| fetch_crate(name, agent.clone()))
            .inspect(|res| {
                if let Err(e) = res {
                    eprintln!("Error getting crate: {e}");
                }
            })
            .filter_map(|res| res.ok());

        println!("Bevy: {bevy_version}");
        for c in crates {
            let readme = match fetch_readme(&c, agent.clone()) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("Error getting readme: {e}");
                    continue;
                }
            };

            let table = match find_support_table(&readme) {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("{e}");
                    continue;
                }
            };

            // currently assuming the bevy column is first
            let mut matching = Vec::new();
            for row in table.iter().map(|r| r.as_slice()) {
                let bevy = extract_version_from_cell(&row[0]);
                let others = extract_versions_from_cell(&row[1]);
                for other in others {
                    if bevy.starts_with(&bevy_version) {
                        matching.push((bevy.clone(), other));
                    }
                }
            }

            if matching.is_empty() {
                eprintln!("{} has no matches for {bevy_version}", c.data.name);
                continue;
            }

            let newest = matching
                .iter()
                .map(|(_, other)| other.parse::<semver::VersionReq>())
                .inspect(|res| {
                    if let Err(e) = res {
                        eprintln!("Failed to parse: {e}");
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

            println!("The most recent version for {} compatible with Bevy {bevy_version} is {newest}", c.data.name);
        }
    }

    println!("Complete");

    Ok(())
}

fn fetch_crate(name: &str, agent: Agent) -> anyhow::Result<CrateResponse> {
    Ok(agent
        .get(&format!("https://crates.io/api/v1/crates/{name}"))
        .call()
        .map_err(|e| anyhow!("Failed to fetch crate data for {name:?}\n{e}"))?
        .into_json::<CrateResponse>()
        .map_err(|e| anyhow!("Failed to parse crate data for {name:?}\n{e}"))?)
}

fn fetch_readme(c: &CrateResponse, agent: Agent) -> anyhow::Result<String> {
    let path = &c.versions[0].readme_path; // index 0 is latest
    Ok(agent
        .get(&format!("https://crates.io{path}"))
        .call()
        .map_err(|e| anyhow!("Failed to fetch readme\n{e}"))?
        .into_string()
        .map_err(|e| anyhow!("Failed to read readme\n{e}"))?)
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
        .filter(|&c| c.is_digit(10) || c == '.')
        .collect()
}

fn extract_versions_from_cell(input: &str) -> Vec<String> {
    input
        .split([' ', ',', '-'])
        .map(|s| s.split(".."))
        .flatten()
        .map(extract_version_from_cell)
        .filter(|s| !s.is_empty())
        .collect()
}

#[derive(serde::Deserialize)]
struct CrateResponse {
    #[serde(rename = "crate")]
    data: CrateDataResponse,
    versions: Vec<CrateVersionResponse>,
}

#[derive(serde::Deserialize)]
struct CrateDataResponse {
    name: String,
}

#[derive(serde::Deserialize)]
struct CrateVersionResponse {
    #[serde(rename = "num")]
    version: String,
    readme_path: String,
}
