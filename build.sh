#!/bin/bash

wasm-pack build --target nodejs --out-dir build-wasm
cargo test
mv bindings src/
tsc
