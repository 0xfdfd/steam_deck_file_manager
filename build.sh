#!/bin/bash

CARGO_BUILD_FLAGS=""
WASM_PACK_BUILD_FLAGS="--dev"
BUILD_BACKEND=true
BUILD_FRONTEND=true
RUN=false

while test $# -gt 0; do
  case "$1" in
    -h|--help)
      echo "Usage: build.sh [OPTION]"
      echo ""
      echo "Options:"
      echo "      --release"
      echo "        Build with release mode."
      echo "      --frontend"
      echo "        Only build frontend."
      echo "      --run"
      echo "        Also run web server."
      echo "  -h, --help"
      echo "        Print help."
      exit 0
      ;;

    --release)
      shift
      CARGO_BUILD_FLAGS="--release"
      WASM_PACK_BUILD_FLAGS="--release"
      ;;

    --frontend)
      shift
      BUILD_BACKEND=false
      BUILD_FRONTEND=true
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
if [[ "${BUILD_FRONTEND}" = true ]]; then
  (cd frontend && cargo fmt && wasm-pack build ${WASM_PACK_BUILD_FLAGS} --no-typescript --no-pack --target web)
fi

# Build backend
if [[ "${BUILD_BACKEND}" = true ]]; then
  (cargo fmt && cargo build ${CARGO_BUILD_FLAGS})
fi

# Run
if [[ "${RUN}" = "true" ]]; then
  (cargo run ${CARGO_BUILD_FLAGS})
fi

