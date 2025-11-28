#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

usage() {
  cat <<'EOF'
Usage: run_all.sh [--help]

Runs both benchmarks in sequence by invoking `run_single.sh` and `run_monte.sh`.
Any environment variables supported by those scripts (PI_ITERATIONS, PI_SAMPLES,
PI_THREADS, PI_SEED) are honored.

Examples:
  ./scripts/run_all.sh
  PI_THREADS=4 PI_SAMPLES=100000000 ./scripts/run_all.sh
EOF
}

if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
  usage
  exit 0
fi

"${SCRIPT_DIR}/run_single.sh"
echo
"${SCRIPT_DIR}/run_monte.sh"
