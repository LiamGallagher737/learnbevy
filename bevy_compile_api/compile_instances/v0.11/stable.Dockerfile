FROM rust:1.74-alpine3.18

RUN rustup target install wasm32-unknown-unknown
RUN apk add --no-cache binaryen alsa-lib-dev eudev-dev musl-dev
RUN cargo install wasm-bindgen-cli

WORKDIR /compile
RUN cargo init --name game --vcs none
COPY ./BaseCargo.toml ./Cargo.toml
RUN cargo build --release --target wasm32-unknown-unknown

RUN rm src/*.rs
RUN rm ./target/wasm32-unknown-unknown/release/deps/game*

COPY ./build.sh ./build.sh
RUN chmod +x build.sh
