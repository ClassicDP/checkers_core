#!/bin/bash

wasm-pack build --target nodejs --out-dir build-wasm
#cargo test
#rm -fr src/bindings
#mv bindings src/
#tsc
