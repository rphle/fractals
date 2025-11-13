#!/bin/bash

cd rust/fractal

# wasm-pack build --target web --release
wasm-pack build --target web --dev


cd ../..

cp -R rust/fractal/pkg/. ui/static/fractal
