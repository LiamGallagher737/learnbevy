#!/bin/bash

set -e
cargo b --release --target wasm32-unknown-unknown --jobs 1
wasm-bindgen --no-typescript --out-dir ./src/ --target web ./target/wasm32-unknown-unknown/release/game.wasm
