#!/bin/bash

# Install dependency
if ! [ -x "$(command -v wasm-opt)" ]; then
  cargo install wasm-opt
fi
if ! [ -x "$(command -v wasm-pack)" ]; then
  cargo install wasm-pack
fi
if ! [ -x "$(command -v wasm-bindgen)" ]; then
  cargo install wasm-bindgen-cli
fi

# Build frontend
(cd frontend && cargo fmt && wasm-pack build --target web)

cp frontend/pkg/frontend.js backend/assets/
cp frontend/pkg/frontend_bg.wasm backend/assets/

# Build backend
(cd backend && cargo fmt && cargo build --release)
