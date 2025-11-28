#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="${SCRIPT_DIR%/scripts}"

ITERATIONS="${PI_ITERATIONS:-50000000}"

usage() {
  cat <<'EOF'
Usage: run_single.sh [--help]

Runs the single-threaded Leibniz PI benchmark via `cargo run --release`.

Environment variables:
  PI_ITERATIONS   Number of iterations (default: 50000000)

Examples:
  ./scripts/run_single.sh
  PI_ITERATIONS=10000000 ./scripts/run_single.sh
EOF
}

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
  usage
  exit 0
fi

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo not found. Install Rust from https://rustup.rs/ first." >&2
  exit 1
fi

cd "${PROJECT_ROOT}"
cargo run --release -- single --iterations "${ITERATIONS}"
