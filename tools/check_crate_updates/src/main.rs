use cargo_toml::Manifest;
use std::{env, fs, io};

fn main() -> io::Result<()> {
    let Some(manifest_dir) = env::args().nth(1) else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Manifest directory must be provided",
        ));
    };

    println!("Searching {manifest_dir}");

    let _ = fs::read_dir(&manifest_dir)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.to_string_lossy().ends_with(".Cargo.toml"))
        .inspect(|path| println!("Found manifest: {path:?}"))
        .map(|path| (Manifest::from_path(&path), path))
        .inspect(|(result, path)| {
            if let Err(e) = result {
                eprintln!("Failed to parse manifest: {path:?}\n{e}");
            }
        })
        .filter_map(|(result, p)| result.map(|m| (m, p)).ok())
        .for_each(|m| ());

    println!("Complete");

    Ok(())
}
