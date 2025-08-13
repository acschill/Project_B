#!/usr/bin/env bash
set -euo pipefail

# ---- Sanity checks -----------------------------------------------------------
if ! uname -m | grep -qE 'aarch64|arm64'; then
  echo "This script targets Raspberry Pi OS 64-bit (aarch64)."
  echo "Detected: $(uname -m). Proceed at your own risk."
fi

# ---- System packages ---------------------------------------------------------
sudo apt-get update
sudo apt-get install -y \
  build-essential \
  cmake \
  ninja-build \
  pkg-config \
  git \
  curl \
  protobuf-compiler \
  libssl-dev

# ---- Rust toolchain via rustup ----------------------------------------------
if ! command -v rustup >/dev/null 2>&1; then
  curl https://sh.rustup.rs -sSf | sh -s -- -y
  # shellcheck disable=SC1090
  source "$HOME/.cargo/env"
fi

rustup default stable
rustup component add rustfmt clippy

# (Pi 5 builds natively; aarch64 target is already the host)
# rustup target add aarch64-unknown-linux-gnu  # not required on the Pi itself

# ---- Verify -----------------------------------------------------------------
echo
echo "== Tool versions =="
rustc --version
cargo --version
cmake --version
protoc --version || echo "protoc not found"
echo
echo "Done. You can now build the Rust (cargo) and C++ (cmake/ninja) modules."

