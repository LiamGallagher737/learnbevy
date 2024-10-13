#![allow(dead_code)]

use std::{env, io, path};
use tokio::{fs, process};

/// An instance for interfacing with the podman containers.
pub struct Instance<'a> {
    /// The name of the image to use.
    image: &'a str,
    /// The commands to run in the container.
    commands: &'a [&'a str],
    /// The code to run the commands on.
    code: &'a str,

    /// A unique ID for this instance.
    unique_id: u128,
    /// The local temp directory for the instance files.
    bind_dir: path::PathBuf,
}

impl<'a> Instance<'a> {
    /// Create a new instance.
    ///
    /// This will create a local directory to be used with the container.
    pub async fn new(image: &'a str, commands: &'a [&'a str], code: &'a str) -> io::Result<Self> {
        let unique_id = fastrand::u128(..);
        let bind_dir = env::temp_dir()
            .join("learnbevy-server-instances")
            .join(unique_id.to_string());

        fs::create_dir_all(&bind_dir).await?;
        fs::write(bind_dir.join("main.rs"), code).await?;

        Ok(Self {
            image,
            commands,
            code,
            unique_id,
            bind_dir,
        })
    }

    /// Execute the comtainer with the given inputs.
    pub async fn execute(&self) -> io::Result<std::process::Output> {
        process::Command::new("podman")
            .args([
                "run",
                "--name",
                &self.unique_id.to_string(),
                "-v",
                &format!("{}:/playground/src/:z", self.bind_dir.display()),
                "--quiet",
                "--pull",
                "never",
                self.image,
            ])
            .args(self.commands)
            .output()
            .await
    }

    /// Read a file in the instance's directory as a [`Vec<u8>`].
    pub async fn read<P: AsRef<path::Path>>(&self, path: P) -> io::Result<Vec<u8>> {
        fs::read(self.bind_dir.join(path)).await
    }

    /// Read a file in the instance's directory as a [`String`].
    pub async fn read_to_string<P: AsRef<path::Path>>(&self, path: P) -> io::Result<String> {
        fs::read_to_string(self.bind_dir.join(path)).await
    }
}

impl Drop for Instance<'_> {
    fn drop(&mut self) {
        let dir = self.bind_dir.clone();
        tokio::spawn(async move { fs::remove_dir_all(dir).await });
    }
}
