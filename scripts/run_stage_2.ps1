# Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
# Licensed under the MIT License
# SPDX-License-Identifier: MIT

$ErrorActionPreference = "Stop"
Push-Location (Split-Path $PSScriptRoot -Parent)

# A tool that scores testability should be held to the number it reports.
# Runs the freshly built binary rather than whatever version happens to be
# installed, so the gate reflects the working tree.
#
# The floor is a ratchet: raise it when the score improves, never lower it to
# turn a red build green. `--threshold` is deliberately not used, because
# `handle_output` returns before the reporter runs, so a threshold failure
# prints no score to explain itself.
function Invoke-Grip4RustSelfGate {
    param(
        [string]$Label = "grip self-analysis",
        [int]$MinScore
    )

    Write-Host "$Label..." -ForegroundColor Cyan

    $previousErrorActionPreference = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    # Analyse core, not the workspace root: fixture/ holds deliberately sloppy
    # sample code that is analysis input, never this tool's own source.
    $corePath = (Resolve-Path (Join-Path $PSScriptRoot "..\core")).Path
    $output = & cargo run --quiet -p cargo-grip4rust -- $corePath --json
    $exitCode = $LASTEXITCODE
    $ErrorActionPreference = $previousErrorActionPreference

    if ($exitCode -ne 0) {
        Write-Host "`nFailed: $Label (exit code $exitCode)" -ForegroundColor Red
        Pop-Location
        exit 1
    }

    $score = (($output -join "`n") | ConvertFrom-Json).overall.grip_score

    if ($null -eq $score) {
        Write-Host "`nFailed: $Label (report carries no overall grip score)" -ForegroundColor Red
        Pop-Location
        exit 1
    }

    Write-Host "  grip score: $score / 100  (floor: $MinScore)"

    if ($score -lt $MinScore) {
        Write-Host "`nFailed: $Label (score $score is below the floor of $MinScore)" -ForegroundColor Red
        Pop-Location
        exit 1
    }
}

function Invoke-Twin4RustGate {
    param(
        [string]$Label,
        [string[]]$Packages
    )

    Write-Host "$Label..." -ForegroundColor Cyan

    if (-not (Get-Command cargo-twin4rust -ErrorAction SilentlyContinue)) {
        Write-Host "`ncargo-twin4rust is not installed." -ForegroundColor Red
        Write-Host "Install it with: cargo install cargo-twin4rust" -ForegroundColor Red
        Pop-Location
        exit 1
    }

    $manifestPath = (Resolve-Path (Join-Path $PSScriptRoot "..\core\Cargo.toml")).Path

    $args = @("twin4rust", "--manifest-path", $manifestPath)
    foreach ($package in $Packages) {
        $args += @("--package", $package)
    }

    $previousErrorActionPreference = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    $output = & cargo @args 2>&1
    $ErrorActionPreference = $previousErrorActionPreference
    $exitCode = $LASTEXITCODE
    $output | ForEach-Object { Write-Host $_ }

    if ($exitCode -ne 0) {
        Write-Host "`nFailed: $Label (source files without a mirrored test)" -ForegroundColor Red
        Pop-Location
        exit 1
    }
}

function Invoke-Crap4RustGate {
    param(
        [string]$Label,
        [string[]]$Packages,
        [string]$Features = "",
        [switch]$NoDefaultFeatures,
        [switch]$IncludeTestTargets,
        [double]$Threshold = 15,
        [switch]$UseProjectThreshold,
        [string[]]$ExcludePaths = @()
    )

    Write-Host "$Label..." -ForegroundColor Cyan

    $manifestPath = (Resolve-Path (Join-Path $PSScriptRoot "..\core\Cargo.toml")).Path
    $args = @("--manifest-path", $manifestPath)
    foreach ($package in $Packages) {
        $args += @("--package", $package)
    }
    if ($Features -ne "") {
        $args += @("--features", $Features)
    }
    if ($NoDefaultFeatures) {
        $args += "--no-default-features"
    }
    if ($IncludeTestTargets) {
        $args += "--include-test-targets"
    }
    foreach ($excludePath in $ExcludePaths) {
        $args += @("--exclude-path", $excludePath)
    }
    $args += @("--warn-only", "--threshold", $Threshold.ToString())

    $previousErrorActionPreference = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    $output = & cargo crap4rust @args 2>&1
    $ErrorActionPreference = $previousErrorActionPreference
    $exitCode = $LASTEXITCODE
    $output | ForEach-Object { Write-Host $_ }

    if ($exitCode -ne 0) {
        Write-Host "`nFailed: $Label (exit code $exitCode)" -ForegroundColor Red
        Pop-Location
        exit 1
    }

    $summaryLine = $output | Select-String -Pattern "summary:\s+total_functions=.*crappy_functions=(\d+)"
    if (-not $summaryLine) {
        Write-Host "`nFailed: $Label (could not parse crap4rust summary)" -ForegroundColor Red
        Pop-Location
        exit 1
    }

    $crappyCount = [int]$summaryLine.Matches[0].Groups[1].Value

    if ($UseProjectThreshold) {
        $verdictLine = $output | Select-String -Pattern "verdict=(clean|warn|crappy)"
        if (-not $verdictLine) {
            Write-Host "`nFailed: $Label (could not parse crap4rust verdict)" -ForegroundColor Red
            Pop-Location
            exit 1
        }
        $verdict = $verdictLine.Matches[0].Groups[1].Value
        if ($verdict -eq "crappy") {
            Write-Host "`nFailed: $Label (project verdict is crappy)" -ForegroundColor Red
            Pop-Location
            exit 1
        }
    } else {
        if ($crappyCount -gt 0) {
            Write-Host "`nFailed: $Label ($crappyCount crappy functions detected)" -ForegroundColor Red
            Pop-Location
            exit 1
        }
    }
}

# ---------------------------------------------------------------------------
# Self-analysis: grip on grip
# ---------------------------------------------------------------------------

function Invoke-Iceberg4RustGate {
    param(
        [string]$Label,
        [string[]]$Packages,
        [string]$Threshold
    )

    Write-Host "$Label..." -ForegroundColor Cyan

    if (-not (Get-Command cargo-iceberg4rust -ErrorAction SilentlyContinue)) {
        Write-Host "`ncargo-iceberg4rust is not installed." -ForegroundColor Red
        Write-Host "Install it with: cargo install cargo-iceberg4rust" -ForegroundColor Red
        Pop-Location
        exit 1
    }

    $manifestPath = (Resolve-Path (Join-Path $PSScriptRoot "..\core\Cargo.toml")).Path

    # The ceiling is passed as a string rather than a [double] so it reaches the
    # CLI unchanged. Interpolating a [double] formats it with the current culture,
    # which emits a comma decimal separator on some locales and fails to parse.
    $args = @("iceberg4rust", "--manifest-path", $manifestPath, "--threshold", $Threshold)
    foreach ($package in $Packages) {
        $args += @("--package", $package)
    }

    $previousErrorActionPreference = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    $output = & cargo @args 2>&1
    $ErrorActionPreference = $previousErrorActionPreference
    $exitCode = $LASTEXITCODE
    $output | ForEach-Object { Write-Host $_ }

    # 2 is the tool's own "offenders found"; anything else non-zero means it
    # could not run at all.
    if ($exitCode -eq 2) {
        Write-Host "`nFailed: $Label (file at or above the ceiling of $Threshold)" -ForegroundColor Red
        Pop-Location
        exit 1
    }
    if ($exitCode -ne 0) {
        Write-Host "`nFailed: $Label (exit code $exitCode)" -ForegroundColor Red
        Pop-Location
        exit 1
    }
}

Invoke-Grip4RustSelfGate -MinScore 59

# ---------------------------------------------------------------------------
# CRAP gate
# ---------------------------------------------------------------------------

Invoke-Crap4RustGate "CRAP cargo-grip4rust" @("cargo-grip4rust")

# ---------------------------------------------------------------------------
# Mirrored test gate
#
# The fixture crates under tests/fixtures are analysis inputs, not sources of
# this crate, so they never reach twin4rust: it resolves source roots from this
# package's own cargo targets.
# ---------------------------------------------------------------------------

Invoke-Twin4RustGate "Mirrored tests cargo-grip4rust" @("cargo-grip4rust")

# ---------------------------------------------------------------------------
# File risk gate
# ---------------------------------------------------------------------------

Invoke-Iceberg4RustGate "File risk cargo-grip4rust" @("cargo-grip4rust") -Threshold "20"

# ---------------------------------------------------------------------------

Write-Host "`ngrip Stage 2 passed!" -ForegroundColor Green
Pop-Location
exit 0
