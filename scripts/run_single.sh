#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="${SCRIPT_DIR%/scripts}"

ITERATIONS="${PI_ITERATIONS:-50000000}"

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo not found. Install Rust from https://rustup.rs/ first." >&2
  exit 1
fi

cd "${PROJECT_ROOT}"
cargo run --release -- single --iterations "${ITERATIONS}"
