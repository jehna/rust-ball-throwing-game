#!/bin/bash
set -e

cargo build --release --bin client --target wasm32-unknown-unknown
wasm-bindgen --out-name game \
  --out-dir build \
  --target web target/wasm32-unknown-unknown/release/client.wasm

cp public/* build/

python3 -m http.server --directory build