#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="${SCRIPT_DIR%/scripts}"

SAMPLES="${PI_SAMPLES:-150000000000}"
THREADS="${PI_THREADS:-}"
SEED="${PI_SEED:-}"

usage() {
  cat <<'EOF'
Usage: run_monte.sh [--help]

Runs the multi-threaded Monte Carlo PI benchmark via `cargo run --release`.

Environment variables:
  PI_SAMPLES   Total random points to generate (default: 150000000000)
  PI_THREADS   Optional thread count override (default: system parallelism)
  PI_SEED      Optional RNG seed for reproducibility

Examples:
  ./scripts/run_monte.sh
  PI_THREADS=8 PI_SAMPLES=50000000 ./scripts/run_monte.sh
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

cmd=(cargo run --release -- monte --samples "${SAMPLES}")
if [[ -n "${THREADS}" ]]; then
  cmd+=(--threads "${THREADS}")
fi
if [[ -n "${SEED}" ]]; then
  cmd+=(--seed "${SEED}")
fi

"${cmd[@]}"
