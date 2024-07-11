use cargo_toml::Manifest;
use std::{env, fs, io, path};
use ureq::{Agent, AgentBuilder};

const EXCLUDE_CRATES: &[&str] = &["bevy", "rand", "rand_chacha", "wasm-bindgen"];

fn main() -> io::Result<()> {
    let Some(manifest_dir) = env::args().nth(1) else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Manifest directory must be provided",
        ));
    };

    let agent = AgentBuilder::new()
        // Crates.io asks that you add a user agent with some way of contacting you
        // https://crates.io/data-access#api
        .user_agent("github.com/LiamGallagher737/learnbevy")
        .build();

    println!("Searching {manifest_dir}");

    let _ = fs::read_dir(&manifest_dir)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.to_string_lossy().ends_with(".Cargo.toml"))
        .inspect(|path| println!("Found manifest: {path:?}"))
        .filter_map(parse_manifest)
        .map(|data| {
            data.manifest
                .dependencies
                .iter()
                .filter(|(name, _)| !EXCLUDE_CRATES.contains(&name.as_str()))
                .map(|(name, _)| fetch_crate_data(name, agent.clone()))
                .collect::<Vec<_>>()
        })
        .for_each(|_| ());

    println!("Complete");

    Ok(())
}

struct ManifestData {
    path: path::PathBuf,
    manifest: Manifest,
}

fn parse_manifest(path: path::PathBuf) -> Option<ManifestData> {
    match Manifest::from_path(&path) {
        Ok(manifest) => Some(ManifestData { path, manifest }),
        Err(e) => {
            eprintln!("Failed to parse {path:?}\n{e}");
            None
        }
    }
}

struct CrateData {
    versions: Vec<String>,
    readme: String,
}

fn fetch_crate_data(name: &str, agent: Agent) -> Option<CrateData> {
    let mut c = agent
        .get(&format!("https://crates.io/api/v1/crates/{name}"))
        .call()
        .inspect_err(|e| eprintln!("Failed to fetch crate data for {name:?}\n{e}"))
        .ok()?
        .into_json::<CrateResponse>()
        .inspect_err(|e| eprintln!("Failed to parse crate data for {name:?}\n{e}"))
        .ok()?;

    let readme_path = &c.versions[0].readme_path; // index 0 is latest

    let readme = agent
        .get(&format!("https://crates.io{readme_path}"))
        .call()
        .inspect_err(|e| eprintln!("Failed to fetch readme for {name:?}\n{e}"))
        .ok()?
        .into_string()
        .inspect_err(|e| eprintln!("Failed to read readme for {name:?}\n{e}"))
        .ok()?;

    println!("Fetched all data for {name}");
    let versions = c.versions.drain(..).map(|v| v.version).collect();
    Some(CrateData { versions, readme })
}

#[derive(serde::Deserialize)]
struct CrateResponse {
    versions: Vec<CrateVersionResponse>,
}

#[derive(serde::Deserialize)]
struct CrateVersionResponse {
    #[serde(rename = "num")]
    version: String,
    readme_path: String,
}
