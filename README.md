# PI Benchmark (Rust)

Simple cross-platform PI benchmark written in Rust with two modes:
- **Single-threaded Leibniz series**: CPU-bound floating point workload.
- **Multi-threaded Monte Carlo**: Embarrassingly parallel random sampling workload.

This is a hobby project meant for learning. Results are not intended for professional benchmarking.

## Requirements
- Rust toolchain (tested with stable, install via https://rustup.rs/)
- macOS or Linux

## Quick start
```bash
# Single-threaded Leibniz (default 50,000,000 iterations)
cargo run --release -- single

# Monte Carlo (default 200,000,000 samples, auto thread count)
cargo run --release -- monte
```

## Scripts for easy runs
- `scripts/run_single.sh` — runs the Leibniz benchmark. Override iterations with `PI_ITERATIONS`.
- `scripts/run_monte.sh` — runs the Monte Carlo benchmark. Override samples/threads/seed with `PI_SAMPLES`, `PI_THREADS`, `PI_SEED`.
- `scripts/run_all.sh` — runs both with their defaults.

Examples:
```bash
PI_ITERATIONS=75000000 ./scripts/run_single.sh
PI_SAMPLES=300000000 PI_THREADS=8 ./scripts/run_monte.sh
```

## CLI usage
```
pi-benchmark <mode> [options]

Modes:
  single        Single-threaded Leibniz series
  monte         Multi-threaded Monte Carlo
```

### Single-threaded options
- `--iterations`, `-n` — number of series iterations (default: 50,000,000).

### Monte Carlo options
- `--samples`, `-s` — total random points to generate (default: 200,000,000).
- `--threads`, `-t` — worker threads (default: system parallelism).
- `--seed` — RNG seed for reproducible runs.

## What the modes do
- **Leibniz**: Computes `pi` via `4 * Σ (-1)^k / (2k + 1)` in a tight, single-threaded loop. Heavy on floating point and branch prediction.
- **Monte Carlo**: Generates random `(x, y)` pairs in the unit square across multiple threads and counts hits inside the unit circle. Each thread uses an independent LCG-based RNG seed.

## Tips
- Use `--release` for meaningful performance numbers (debug builds are much slower).
- Pin thread counts (`--threads`) when comparing across machines to avoid differences from SMT or OS scheduling.
- For reproducible Monte Carlo runs, supply an explicit `--seed` or `PI_SEED`.

## Project layout
- `src/main.rs` — CLI and benchmark implementations.
- `scripts/` — convenience runners for macOS/Linux shells.
- `Cargo.toml` — crate metadata and release profile tuned for benchmarking (LTO, single codegen unit).

## Testing
```bash
cargo test
```

## Notes
This is a learning tool. For rigorous benchmarking, consider dedicated suites and account for system noise, thermal throttling, and background load.
