Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

param(
    [switch]$Help
)

function Show-Usage {
@'
Usage: run_single.ps1 [-Help]

Runs the single-threaded Leibniz PI benchmark via `cargo run --release`.

Environment variables:
  PI_ITERATIONS   Number of iterations (default: 30000000000)

Examples:
  .\scripts\run_single.ps1
  $env:PI_ITERATIONS=10000000; .\scripts\run_single.ps1
'@
}

if ($Help -or ($args -and $args[0] -in @("-h", "--help"))) {
    Show-Usage
    exit 0
}

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$projectRoot = Split-Path -Parent $scriptDir
$iterations = if ($env:PI_ITERATIONS) { $env:PI_ITERATIONS } else { "30000000000" }

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Error "cargo not found. Install Rust from https://rustup.rs/ first."
    exit 1
}

Push-Location $projectRoot
try {
    & cargo run --release -- single --iterations $iterations
} finally {
    Pop-Location
}
