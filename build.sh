#!/bin/bash
set -e

# Build Rust WASM
cd rust/fractal
wasm-pack build --target web --dev --out-dir ../../ui/static/fractal

# Build Svelte UI
cd ../../ui
tree static
npm install
npm run build
