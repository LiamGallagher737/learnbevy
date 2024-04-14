# Based off https://github.com/rust-lang/rust-playground/blob/04e0f7c773415946413e7c79a77d71a796f175ef/compiler/base/Dockerfile

FROM ubuntu:20.04 as toolchain

ARG version
ARG channel

ENV DEBIAN_FRONTEND="noninteractive"

RUN apt-get update && apt-get install -y g++ pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0 curl

RUN useradd -m playground -d /playground
RUN usermod -p '!!' root # Disable all passwords for root

USER playground
ENV USER=playground
ENV PATH=/playground/.cargo/bin:$PATH
WORKDIR /playground

# Ensure that we are using the latest stable version of rustup and the
# latest version of the current channel. A new manifest will trigger
# these lines to run again, forcing a new download of rustup and
# installation of Rust.
ADD --chown=playground https://static.rust-lang.org/rustup/release-stable.toml /playground/tools/rustup-manifest.toml
ADD --chown=playground https://static.rust-lang.org/dist/channel-rust-${channel}-date.txt /playground/tools/rust-channel-version

RUN curl https://sh.rustup.rs -sSf | sh -s -- \
    -y \
    --profile minimal \
    --default-toolchain "${channel}" \
    --target wasm32-unknown-unknown #\
    #--component rust-src
#RUN if [ "${channel}" = 'nightly' ]; then rustup component add miri; fi

COPY --chown=playground build.sh /playground/tools/

FROM toolchain as wasm-bindgen
RUN cargo install wasm-bindgen-cli
RUN ls  -a  .cargo/bin

FROM toolchain as sources
RUN cargo init --name game --vcs none /playground
COPY --chown=playground ${version}.Cargo.toml /playground/Cargo.toml
RUN cargo fetch

FROM sources

ARG channel

RUN cargo build --release --target wasm32-unknown-unknown
RUN rm src/*.rs

COPY --from=wasm-bindgen /playground/.cargo/bin/wasm-bindgen /playground/.cargo/bin

