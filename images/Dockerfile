# Based off https://github.com/rust-lang/rust-playground/blob/04e0f7c773415946413e7c79a77d71a796f175ef/compiler/base/Dockerfile

FROM ubuntu:20.04 as toolchain

ENV DEBIAN_FRONTEND="noninteractive"

RUN apt-get update && apt-get install -y g++ pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0 curl

RUN useradd -m playground -d /playground
RUN usermod -p '!!' root # Disable all passwords for root

ENV PATH=/root/.cargo/bin:$PATH
WORKDIR /playground

ARG channel

# Ensure that we are using the latest stable version of rustup and the
# latest version of the current channel. A new manifest will trigger
# these lines to run again, forcing a new download of rustup and
# installation of Rust.
ADD https://static.rust-lang.org/rustup/release-stable.toml /playground/tools/rustup-manifest.toml
ADD https://static.rust-lang.org/dist/channel-rust-${channel}-date.txt /playground/tools/rust-channel-version

RUN curl https://sh.rustup.rs -sSf | sh -s -- \
    -y \
    --profile minimal \
    --default-toolchain "${channel}" \
    --target wasm32-unknown-unknown 

RUN rustup component add clippy
# For linting
RUN rustup toolchain install nightly-2025-04-03 --component rustc-dev --component llvm-tools-preview
RUN rustup target add --toolchain nightly-2025-04-03 wasm32-unknown-unknown


FROM toolchain as wasm-bindgen
RUN cargo install wasm-bindgen-cli --version 0.2.100


FROM toolchain as bevy_lint
RUN cargo +nightly-2025-04-03 install --git https://github.com/TheBevyFlock/bevy_cli --tag lint-v0.3.0 bevy_lint


FROM toolchain as sources
RUN cargo init --name game --vcs none /playground

ARG version

# Force a rebuild when the main branch is updated
# TODO: Find a way to make this conditional for only main version builds
ADD https://api.github.com/repos/bevyengine/bevy/commits/main /playground/tools/latest-bevy-main

COPY playground_lib/ /playground/playground_lib
COPY manifests/${version}.Cargo.toml /playground/Cargo.toml
COPY ${channel}.config.toml /playground/.cargo/config.toml
RUN cargo fetch


FROM sources

ARG channel

RUN cargo build --release --target wasm32-unknown-unknown
RUN cargo clippy --target wasm32-unknown-unknown

COPY --from=wasm-bindgen /root/.cargo/bin/wasm-bindgen /root/.cargo/bin
COPY --from=bevy_lint /root/.cargo/bin/bevy_lint /root/.cargo/bin
COPY --from=bevy_lint /root/.cargo/bin/bevy_lint_driver /root/.cargo/bin

RUN bevy_lint --target wasm32-unknown-unknown

RUN rm src/*.rs

COPY build.sh /playground/tools/
RUN chmod u+x /playground/tools/build.sh

