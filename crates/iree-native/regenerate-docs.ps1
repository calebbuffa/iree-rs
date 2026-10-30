#Requires -Version 7.0
<#
.SYNOPSIS
Regenerate all hosted API snapshots, verify docs, and verify the crate archive.
.EXAMPLE
.\crates\iree-native\regenerate-docs.ps1 -InstallToolchains -AllowDirty
.NOTES
Never publishes. Requires Rust, Python, CMake, Ninja, Git, host libclang,
and Visual Studio C++ tools. -InstallToolchains provisions cross headers/SDKs
and missing nightly Rust components into the tool cache or rustup.
#>
[CmdletBinding()]
param(
    [switch]$InstallToolchains,
    [switch]$AllowDirty,
    [string]$ZigLib,
    [string]$EmsdkRoot,
    [string]$OutputDirectory,
    [string]$ClangArgumentsFile
)

$ErrorActionPreference = 'Stop'
$crate = $PSScriptRoot
$workspace = Split-Path (Split-Path $crate -Parent) -Parent
if (-not $OutputDirectory) {
    $OutputDirectory = Join-Path $workspace 'target\iree-native-docs-maintenance'
}
$OutputDirectory = [IO.Path]::GetFullPath($OutputDirectory)
$targets = @(
    'x86_64-unknown-linux-gnu',
    'x86_64-pc-windows-msvc',
    'aarch64-apple-darwin',
    'wasm32-unknown-emscripten'
)
$variables = @(
    'CARGO_TARGET_DIR', 'DOCS_RS', 'WOODSHED_OFFLINE', 'IREE_NATIVE_DIR',
    'IREE_NATIVE_CACHE_DIR', 'LIBCLANG_PATH', 'IREE_NATIVE_DOCS_SNAPSHOT_DIR',
    'IREE_NATIVE_DOCS_TARGET', 'IREE_NATIVE_DOCS_CLANG_ARGS'
)
$saved = @{}
foreach ($name in $variables) {
    $saved[$name] = [Environment]::GetEnvironmentVariable($name, 'Process')
}

function Invoke-Checked {
    param([string]$Program, [string[]]$Arguments)
    Write-Host ('> ' + $Program + ' ' + ($Arguments -join ' '))
    & $Program @Arguments
    if ($LASTEXITCODE -ne 0) {
        throw "$Program failed with exit code $LASTEXITCODE"
    }
}

function Get-CheckedOutput {
    param([string]$Program, [string[]]$Arguments)
    $output = & $Program @Arguments
    if ($LASTEXITCODE -ne 0) {
        throw "$Program failed with exit code $LASTEXITCODE"
    }
    return ($output -join "`n").Trim()
}

$lock = $null
Push-Location $workspace
try {
    if (-not $IsWindows) {
        throw 'Run this maintainer command on Windows with the MSVC and Windows SDK headers installed.'
    }
    foreach ($program in @('cargo', 'rustup', 'python', 'cmake', 'ninja', 'git', 'clang')) {
        if (-not (Get-Command $program -ErrorAction SilentlyContinue)) {
            throw "Missing $program. Install the native build prerequisites listed in crates\iree-native\README.md."
        }
    }
    if (-not $env:LIBCLANG_PATH) {
        $clangDirectory = Split-Path (Get-Command clang).Source -Parent
        if (Test-Path (Join-Path $clangDirectory 'libclang.dll')) {
            $env:LIBCLANG_PATH = $clangDirectory
        }
    }
    New-Item -ItemType Directory -Force $OutputDirectory | Out-Null
    $lock = [IO.File]::Open(
        (Join-Path $OutputDirectory 'maintenance.lock'),
        [IO.FileMode]::OpenOrCreate, [IO.FileAccess]::ReadWrite, [IO.FileShare]::None
    )
    $tools = Join-Path $OutputDirectory 'tools'
    New-Item -ItemType Directory -Force $tools | Out-Null
    $overrides = @{}
    if ($ClangArgumentsFile) {
        $overrides = Get-Content $ClangArgumentsFile -Raw | ConvertFrom-Json -AsHashtable
    }
    if (-not $ZigLib) { $ZigLib = $env:IREE_NATIVE_DOCS_ZIG_LIB }
    if (-not $ZigLib -and (Get-Command zig -ErrorAction SilentlyContinue)) {
        $ZigLib = (Get-CheckedOutput 'zig' @('env') | ConvertFrom-Json).lib_dir
    }
    if (-not $ZigLib) {
        $cached = Join-Path $tools 'zig-headers\lib'
        if (Test-Path (Join-Path $cached 'libc\include')) { $ZigLib = $cached }
    }
    $needsZig = (-not $overrides.ContainsKey('x86_64-unknown-linux-gnu')) -or
        (-not $overrides.ContainsKey('aarch64-apple-darwin'))
    if ($needsZig -and -not $ZigLib -and $InstallToolchains) {
        $release = (Invoke-RestMethod 'https://ziglang.org/download/index.json').'0.15.2'.src
        if (-not $release.tarball -or -not $release.shasum) { throw 'Missing Zig 0.15.2 source metadata.' }
        $archive = Join-Path $tools 'zig-0.15.2.tar.xz'
        Invoke-WebRequest $release.tarball -OutFile $archive
        if ((Get-FileHash $archive -Algorithm SHA256).Hash.ToLowerInvariant() -ne $release.shasum) {
            throw 'Zig source archive checksum mismatch.'
        }
        $headers = Join-Path $tools 'zig-headers'
        New-Item -ItemType Directory -Force $headers | Out-Null
        Invoke-Checked 'tar' @('-xf', $archive, '-C', $headers, '--strip-components=1', '--include', '*libc*include*')
        $ZigLib = Join-Path $headers 'lib'
    }
    if ($needsZig -and -not $ZigLib) {
        throw 'Linux/macOS headers missing. Use -InstallToolchains, -ZigLib <zig\lib>, or -ClangArgumentsFile.'
    }
    if (-not $EmsdkRoot) { $EmsdkRoot = $env:EMSDK }
    $emcc = Get-Command emcc, emcc.bat -ErrorAction SilentlyContinue | Select-Object -First 1
    if (-not $EmsdkRoot -and $emcc) {
        $EmsdkRoot = Split-Path (Split-Path (Split-Path $emcc.Source -Parent) -Parent) -Parent
    }
    if (-not $EmsdkRoot -and (Test-Path (Join-Path $tools 'emsdk\emsdk.bat'))) {
        $EmsdkRoot = Join-Path $tools 'emsdk'
    }
    if (-not $overrides.ContainsKey('wasm32-unknown-emscripten') -and $InstallToolchains) {
        if (-not $EmsdkRoot) {
            $EmsdkRoot = Join-Path $tools 'emsdk'
            Invoke-Checked 'git' @('clone', '--quiet', '--depth', '1', 'https://github.com/emscripten-core/emsdk', $EmsdkRoot)
        }
        if (-not (Test-Path (Join-Path $EmsdkRoot 'upstream\emscripten\cache\sysroot\include\emscripten.h'))) {
            Invoke-Checked (Join-Path $EmsdkRoot 'emsdk.bat') @('install', '4.0.10')
        }
    }
    $resource = Get-CheckedOutput 'clang' @('-print-resource-dir')
    $builtin = @('-nostdinc', '-isystem', (Join-Path $resource 'include'))
    $arguments = @{'x86_64-pc-windows-msvc' = @()}
    foreach ($target in @('x86_64-unknown-linux-gnu', 'aarch64-apple-darwin')) {
        if ($overrides.ContainsKey($target)) { continue }
        $arguments[$target] = @($builtin)
        $directories = if ($target -eq 'x86_64-unknown-linux-gnu') {
            @('x86-linux-gnu', 'generic-glibc', 'x86-linux-any', 'any-linux-any')
        } else { @('any-macos-any') }
        foreach ($directory in $directories) {
            $include = Join-Path $ZigLib "libc\include\$directory"
            if (-not (Test-Path $include -PathType Container)) { throw "Missing target headers: $include" }
            $arguments[$target] += @('-isystem', $include)
        }
    }
    if (-not $overrides.ContainsKey('wasm32-unknown-emscripten')) {
        if (-not $EmsdkRoot) { throw 'Emscripten SDK missing. Use -InstallToolchains or -EmsdkRoot.' }
        $sysroot = Join-Path $EmsdkRoot 'upstream\emscripten\cache\sysroot'
        if (-not (Test-Path (Join-Path $sysroot 'include\emscripten.h'))) {
            throw "Missing Emscripten sysroot: $sysroot"
        }
        $arguments['wasm32-unknown-emscripten'] = $builtin +
            @("--sysroot=$sysroot", '-isystem', (Join-Path $sysroot 'include'))
    }
    foreach ($target in $overrides.Keys) {
        if ($target -notin $targets -or $overrides[$target] -isnot [array] -or
            @($overrides[$target] | Where-Object { $_ -isnot [string] }).Count) {
            throw "Invalid target or SDK argument array in ClangArgumentsFile: $target"
        }
        $arguments[$target] = $overrides[$target]
    }
    $installed = Get-CheckedOutput 'rustup' @('toolchain', 'list')
    if ($installed -notmatch '(?m)^nightly-') {
        if (-not $InstallToolchains) { throw 'Nightly Rust missing. Use -InstallToolchains or rustup toolchain install nightly.' }
        Invoke-Checked 'rustup' @('toolchain', 'install', 'nightly', '--profile', 'minimal')
    }
    $installedTargets = (Get-CheckedOutput 'rustup' @('target', 'list', '--installed', '--toolchain', 'nightly')) -split "`n"
    $missing = @($targets | Where-Object { $_ -notin $installedTargets })
    if ($missing.Count) {
        if (-not $InstallToolchains) {
            throw "Missing nightly targets: $($missing -join ', '). Use -InstallToolchains."
        }
        Invoke-Checked 'rustup' (@('target', 'add', '--toolchain', 'nightly') + $missing)
    }
    $env:CARGO_TARGET_DIR = $OutputDirectory
    Remove-Item Env:DOCS_RS -ErrorAction SilentlyContinue
    $staging = Join-Path $OutputDirectory ('snapshots-' + [Guid]::NewGuid().ToString('N'))
    foreach ($target in $targets) {
        $env:IREE_NATIVE_DOCS_SNAPSHOT_DIR = $staging
        $env:IREE_NATIVE_DOCS_TARGET = $target
        $env:IREE_NATIVE_DOCS_CLANG_ARGS = ConvertTo-Json -Compress -InputObject @($arguments[$target])
        Invoke-Checked 'cargo' @('check', '-p', 'iree-native', '--features', 'compiler,compiler-tools')
    }
    # Do not replace checked-in snapshots until every target has generated.
    foreach ($target in $targets) {
        $source = Join-Path $staging $target
        $metadata = Get-Content (Join-Path $source 'snapshot.json') -Raw | ConvertFrom-Json -AsHashtable
        if ($metadata.generation_target -ne $target) { throw "Snapshot target mismatch for $target" }
        $destination = Join-Path $crate "docs\bindings\$target"
        New-Item -ItemType Directory -Force $destination | Out-Null
        foreach ($name in $metadata.files.Keys) {
            if ([IO.Path]::GetFileName($name) -ne $name -or $name -notlike '*.rs') {
                throw "Invalid snapshot file: $name"
            }
            Copy-Item (Join-Path $source $name) (Join-Path $destination $name)
        }
        Copy-Item (Join-Path $source 'snapshot.json') (Join-Path $destination 'snapshot.json')
    }
    foreach ($name in @('IREE_NATIVE_DOCS_SNAPSHOT_DIR', 'IREE_NATIVE_DOCS_TARGET', 'IREE_NATIVE_DOCS_CLANG_ARGS')) {
        Remove-Item "Env:$name" -ErrorAction SilentlyContinue
    }
    Invoke-Checked 'cargo' @('test', '-p', 'iree-native', '--test', 'documentation')
    $packageArguments = @('package', '-p', 'iree-native')
    if ($AllowDirty) { $packageArguments += '--allow-dirty' }
    Invoke-Checked 'cargo' $packageArguments
    $metadata = Get-CheckedOutput 'cargo' @('metadata', '--no-deps', '--format-version', '1') | ConvertFrom-Json
    $version = ($metadata.packages | Where-Object name -EQ 'iree-native').version
    $manifest = Join-Path $OutputDirectory "package\iree-native-$version\Cargo.toml"
    $env:DOCS_RS = '1'
    $env:WOODSHED_OFFLINE = '1'
    $env:IREE_NATIVE_DIR = Join-Path $OutputDirectory 'deliberately-missing-sdk'
    $env:IREE_NATIVE_CACHE_DIR = Join-Path $OutputDirectory 'deliberately-empty-native-cache'
    $env:LIBCLANG_PATH = Join-Path $OutputDirectory 'deliberately-missing-libclang'
    foreach ($target in $targets) {
        Invoke-Checked 'cargo' @(
            '+nightly', 'rustdoc', '--manifest-path', $manifest,
            '--all-features', '--target', $target, '--', '--cfg', 'docsrs'
        )
    }
    Write-Host 'All four documentation targets and the extracted crate passed. Nothing was published.'
    Write-Host 'Commit the generated snapshots, then run: cargo publish -p iree-native'
}
finally {
    if ($lock) { $lock.Dispose() }
    foreach ($name in $variables) {
        [Environment]::SetEnvironmentVariable($name, $saved[$name], 'Process')
    }
    Pop-Location
}
