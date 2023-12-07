#!/bin/bash

cargo b --release --target wasm32-unknown-unknown --jobs 1 --debug-assertions false
wasm-bindgen --out-dir ./src/ --target web ./target/wasm32-unknown-unknown/release/game.wasm
# mv /compile/target/wasm32-unknown-unknown/release/game.wasm /compile/src/game.wasm
# wasm-opt -Oz -o /compile/src/game.wasm /compile/src/game.wasm
