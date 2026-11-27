$ErrorActionPreference = 'Stop'
$timings = [ordered]@{}
$overall = [Diagnostics.Stopwatch]::StartNew()

function Invoke-Timed([string] $Name, [scriptblock] $Command) {
    $timer = [Diagnostics.Stopwatch]::StartNew()
    try {
        & $Command
        if ($LASTEXITCODE -ne 0) {
            throw "$Name failed with exit code $LASTEXITCODE"
        }
    } finally {
        $timer.Stop()
        $timings[$Name] = [Math]::Round($timer.Elapsed.TotalSeconds, 1)
    }
}

try {
    if ($env:BUILD_MODE -eq 'FAST') {
        if ($env:RUSTC_WRAPPER -ne 'sccache') {
            throw 'FAST mode requires the sccache Rust compiler wrapper'
        }
        sccache --zero-stats
        if ($LASTEXITCODE -ne 0) { throw 'Unable to start sccache' }
    }
    $setupSeconds = [DateTimeOffset]::UtcNow.ToUnixTimeSeconds() - [long]$env:CI_START
    $package = (cargo metadata --no-deps --format-version 1 --locked | ConvertFrom-Json).packages |
        Where-Object name -eq 'aeris-core'
    if ($LASTEXITCODE -ne 0 -or -not $package.rust_version) {
        throw 'Unable to determine the minimum supported Rust version'
    }
    $minimum = $package.rust_version
    $rustVersion = (rustc --version)
    $cargoVersion = (cargo --version)
    Invoke-Timed 'Minimum Rust' {
        rustup toolchain install $minimum --profile minimal
        if ($LASTEXITCODE -ne 0) { throw 'Minimum Rust installation failed' }
        cargo "+$minimum" check --workspace --all-targets --locked
    }
    Invoke-Timed 'Formatting' { cargo fmt --all -- --check }
    Invoke-Timed 'Clippy' { cargo clippy --workspace --all-targets --locked -- -D warnings }
    Invoke-Timed 'Tests' { cargo test --workspace --locked }
    Invoke-Timed 'Release build' { cargo build --workspace --release --locked --target x86_64-pc-windows-msvc }
    Invoke-Timed 'CLI smoke' {
        $root = Join-Path $env:RUNNER_TEMP 'aeris-smoke'
        New-Item -ItemType Directory -Force (Join-Path $root 'cli') | Out-Null
        Copy-Item target/x86_64-pc-windows-msvc/release/aeris-core.exe $root
        Copy-Item cli/aeris.cmd (Join-Path $root 'cli')
        $env:PATH = (Join-Path $root 'cli') + ';' + $env:PATH
        Push-Location $env:RUNNER_TEMP
        try {
            $expected = & (Join-Path $root 'aeris-core.exe') version
            if ($LASTEXITCODE -ne 0 -or $expected -notmatch '^Aeris Core \S+$') {
                throw 'Core version check failed'
            }
            $result = aeris version
            if ($LASTEXITCODE -ne 0 -or $result -ne $expected) {
                throw 'CLI smoke test failed'
            }
        } finally {
            Pop-Location
        }
    }
} finally {
    $overall.Stop()
    $cargoCache = if ($env:BUILD_MODE -eq 'CLEAN') { 'disabled' } elseif ($env:CARGO_CACHE_HIT -eq 'true') { 'exact hit' } else { 'miss' }
    $sccache = if ($env:BUILD_MODE -ne 'FAST') { 'disabled' } elseif ($env:SCCACHE_HIT -eq 'true') { 'exact hit' } elseif ($env:SCCACHE_HIT -eq 'false') { 'compatible fallback' } else { 'miss' }
    @(
        '## Core CI'
        ''
        "- Mode: $env:BUILD_MODE"
        "- Rust: $rustVersion"
        "- Cargo: $cargoVersion"
        "- Setup: $setupSeconds s"
        "- Cargo download cache: $cargoCache"
        "- sccache archive: $sccache"
        '- Component: Core and CLI built together; no prebuilt component restored.'
        '- Packaging: not performed.'
        ''
        '| Stage | Seconds |'
        '| --- | ---: |'
    ) | Add-Content -Encoding utf8 $env:GITHUB_STEP_SUMMARY
    foreach ($stage in $timings.GetEnumerator()) {
        "| $($stage.Key) | $($stage.Value) |" | Add-Content -Encoding utf8 $env:GITHUB_STEP_SUMMARY
    }
    "| Validation/build total | $([Math]::Round($overall.Elapsed.TotalSeconds, 1)) |" |
        Add-Content -Encoding utf8 $env:GITHUB_STEP_SUMMARY
    if ($env:BUILD_MODE -eq 'FAST') {
        '### sccache statistics' | Add-Content -Encoding utf8 $env:GITHUB_STEP_SUMMARY
        if (Get-Command sccache -ErrorAction SilentlyContinue) {
            sccache --version | ForEach-Object { "    $_" } | Add-Content -Encoding utf8 $env:GITHUB_STEP_SUMMARY
            sccache --stop-server | ForEach-Object { "    $_" } | Add-Content -Encoding utf8 $env:GITHUB_STEP_SUMMARY
        } else {
            '    sccache unavailable' | Add-Content -Encoding utf8 $env:GITHUB_STEP_SUMMARY
        }
    }
}
