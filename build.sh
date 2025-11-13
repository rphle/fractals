#!/bin/bash
set -e

# Build Svelte UI
cd ui
npm install
npm run build

# Build Rust WASM
cd ../rust/fractal
wasm-pack build --target web --dev --out-dir ../../ui/static/fractal
cp -r ../../ui/static/fractal ../../ui/build/fractal
