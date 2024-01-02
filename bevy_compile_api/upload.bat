wsl ~/.cargo/bin/cargo b --release --target x86_64-unknown-linux-musl && scp target\x86_64-unknown-linux-musl\release\bevy_compile_api root@45.142.164.20:/server
