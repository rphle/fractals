#!/bin/bash
set -e

# Build Rust WASM
cd rust/fractal
wasm-pack build --target web --dev --out-dir ../../ui/static/fractal
rm ../../ui/static/fractal/.gitignore

# Build Svelte UI
cd ../../ui
npm install
npm run build
