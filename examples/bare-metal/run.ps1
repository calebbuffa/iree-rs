param(
    [string]$ArmGnuRoot = $env:ARM_GNU_ROOT,
    [string]$HostBinDirectory = $env:IREE_NATIVE_HOST_BIN_DIR,
    [string]$Qemu = "qemu-system-arm",
    [string]$BuildDirectory = (Join-Path (Get-Location) "target-bare-metal"),
    [switch]$SkipRun
)
$ErrorActionPreference = "Stop"
if (!$ArmGnuRoot) { throw "Set ARM_GNU_ROOT to an Arm GNU bare-metal SDK." }
$env:ARM_GNU_ROOT = (Resolve-Path -LiteralPath $ArmGnuRoot).Path
$env:PATH = "$env:ARM_GNU_ROOT\bin;$env:PATH"
if ($HostBinDirectory) {
    $env:IREE_NATIVE_HOST_BIN_DIR = (Resolve-Path -LiteralPath $HostBinDirectory).Path
}
$BuildDirectory = [System.IO.Path]::GetFullPath($BuildDirectory)
$consumer = Join-Path $BuildDirectory "consumer"
New-Item -ItemType Directory -Force -Path $consumer, (Join-Path $consumer "src"), (Join-Path $consumer ".cargo") | Out-Null
foreach ($file in @("build.rs", "memory.ld", "model.vmfb", "platform.h", "toolchain.cmake", "Cargo.lock")) {
    Copy-Item -LiteralPath (Join-Path $PSScriptRoot $file) -Destination (Join-Path $consumer $file)
}
foreach ($file in @("main.rs", "board.rs")) {
    Copy-Item -LiteralPath (Join-Path $PSScriptRoot "src\$file") -Destination (Join-Path $consumer "src\$file")
}
$crate = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot "..\..\crates\iree")).Path
$manifest = (Get-Content -LiteralPath (Join-Path $PSScriptRoot "Cargo.toml.in") -Raw).
    Replace("@IREE_CRATE@", (ConvertTo-Json -InputObject $crate -Compress))
Set-Content -LiteralPath (Join-Path $consumer "Cargo.toml") -Value $manifest -Encoding utf8
$toolchain = Join-Path $PSScriptRoot "toolchain.cmake"
$config = (Get-Content -LiteralPath (Join-Path $PSScriptRoot ".cargo\config.toml") -Raw).
    Replace('value = "toolchain.cmake", relative = true', ("value = " + (ConvertTo-Json -InputObject $toolchain -Compress)))
Set-Content -LiteralPath (Join-Path $consumer ".cargo\config.toml") -Value $config -Encoding utf8
if (!$env:CARGO_TARGET_DIR) {
    $env:CARGO_TARGET_DIR = Join-Path $BuildDirectory "cargo"
}
$env:CARGO_TARGET_DIR = [System.IO.Path]::GetFullPath($env:CARGO_TARGET_DIR)
Push-Location $consumer
try {
    cargo build --locked
    if ($LASTEXITCODE) { throw "Cargo firmware build failed ($LASTEXITCODE)." }
    if (!$SkipRun) {
        $firmware = Join-Path $env:CARGO_TARGET_DIR "thumbv7em-none-eabihf\debug\iree-bare-metal"
        & $Qemu -M mps2-an386 -nographic -semihosting-config enable=on,target=native -kernel $firmware
        if ($LASTEXITCODE) { throw "QEMU firmware failed ($LASTEXITCODE)." }
    }
}
finally { Pop-Location }
