#!/bin/bash
wasm-pack build --target nodejs --out-dir build-wasm
rm build-wasm/.gitignore
rm build-wasm/package.json
rm build-wasm/README.md
cargo test
rm -fr src/bindings
mv bindings src/
tsc
