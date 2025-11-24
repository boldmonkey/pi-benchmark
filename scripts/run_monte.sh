#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="${SCRIPT_DIR%/scripts}"

SAMPLES="${PI_SAMPLES:-200000000}"
THREADS="${PI_THREADS:-}"
SEED="${PI_SEED:-}"

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
