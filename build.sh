#!/bin/bash

cd rust/fractal

wasm-pack build --target web --dev

cd ../../ui

cp -R ../rust/fractal/pkg/. static/fractal
npm run build
