Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

param(
    [switch]$Help
)

function Show-Usage {
@'
Usage: run_all.ps1 [-Help]

Runs both benchmarks in sequence by invoking `run_single.ps1` and `run_monte.ps1`.
Any environment variables supported by those scripts (PI_ITERATIONS, PI_SAMPLES,
PI_THREADS, PI_SEED) are honored.

Examples:
  .\scripts\run_all.ps1
  $env:PI_THREADS=4; $env:PI_SAMPLES=100000000; .\scripts\run_all.ps1
'@
}

if ($Help -or ($args -and $args[0] -in @("-h", "--help"))) {
    Show-Usage
    exit 0
}

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path

& (Join-Path $scriptDir "run_single.ps1")
Write-Host ""
& (Join-Path $scriptDir "run_monte.ps1")
