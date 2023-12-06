#!/bin/bash

CARGO_BUILD_FLAGS=""
WASM_PACK_BUILD_FLAGS="--dev"
RUN=false

while test $# -gt 0; do
  case "$1" in
    -h|--help)
      echo "build.sh --release"
      echo ""
      echo "  --release: Build with --release"
      exit 0
      ;;

    --release)
      shift
      CARGO_BUILD_FLAGS="--release"
	  WASM_PACK_BUILD_FLAGS="--release"
      ;;

    --run)
      shift
      RUN=true
      ;;

  esac
done

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
(cd frontend && cargo fmt && wasm-pack build ${WASM_PACK_BUILD_FLAGS} --target web)
cp frontend/pkg/frontend.js backend/assets/
cp frontend/pkg/frontend_bg.wasm backend/assets/

# Build backend
(cd backend && cargo fmt && cargo build ${CARGO_BUILD_FLAGS})

# Run
if [[ "${RUN}" = "true" ]]; then
  (cd backend && cargo run ${CARGO_BUILD_FLAGS})
fi

