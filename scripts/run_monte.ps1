Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

param(
    [switch]$Help
)

function Show-Usage {
@'
Usage: run_monte.ps1 [-Help]

Runs the multi-threaded Monte Carlo PI benchmark via `cargo run --release`.

Environment variables:
  PI_SAMPLES   Total random points to generate (default: 200000000)
  PI_THREADS   Optional thread count override (default: system parallelism)
  PI_SEED      Optional RNG seed for reproducibility

Examples:
  .\scripts\run_monte.ps1
  $env:PI_THREADS=8; $env:PI_SAMPLES=50000000; .\scripts\run_monte.ps1
'@
}

if ($Help -or ($args -and $args[0] -in @("-h", "--help"))) {
    Show-Usage
    exit 0
}

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$projectRoot = Split-Path -Parent $scriptDir

$samples = if ($env:PI_SAMPLES) { $env:PI_SAMPLES } else { "200000000" }
$threads = $env:PI_THREADS
$seed = $env:PI_SEED

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Error "cargo not found. Install Rust from https://rustup.rs/ first."
    exit 1
}

Push-Location $projectRoot
try {
    $cmd = @("cargo", "run", "--release", "--", "monte", "--samples", $samples)
    if ($threads) { $cmd += @("--threads", $threads) }
    if ($seed) { $cmd += @("--seed", $seed) }
    & $cmd
} finally {
    Pop-Location
}
