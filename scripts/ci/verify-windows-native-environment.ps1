$ErrorActionPreference = 'Stop'

function Require-Command([string]$Name) {
  if (-not (Get-Command $Name -ErrorAction SilentlyContinue)) {
    throw "Required command '$Name' was not found on the Windows build runner."
  }
}

Require-Command 'cargo'
Require-Command 'rustc'
Require-Command 'node'
Require-Command 'npm'

$vcpkgRoot = if ($env:COREVIA_WINDOWS_VCPKG_ROOT) {
  $env:COREVIA_WINDOWS_VCPKG_ROOT
} elseif ($env:VCPKG_ROOT) {
  $env:VCPKG_ROOT
} else {
  'C:\vcpkg'
}

$vcpkg = Join-Path $vcpkgRoot 'vcpkg.exe'
if (-not (Test-Path -LiteralPath $vcpkg -PathType Leaf)) {
  throw "vcpkg was not found at '$vcpkg'. Prepare the persistent Windows runner once, then rerun CI."
}

$triplet = if ($env:COREVIA_WINDOWS_VCPKG_TRIPLET) {
  $env:COREVIA_WINDOWS_VCPKG_TRIPLET
} else {
  'x64-windows-static'
}
$installedRoot = if ($env:COREVIA_WINDOWS_VCPKG_INSTALLED_ROOT) {
  $env:COREVIA_WINDOWS_VCPKG_INSTALLED_ROOT
} else {
  Join-Path $vcpkgRoot 'installed'
}
$libRoot = Join-Path (Join-Path $installedRoot $triplet) 'lib'
foreach ($library in @('aom.lib', 'opus.lib', 'vpx.lib', 'yuv.lib')) {
  if (-not (Test-Path -LiteralPath (Join-Path $libRoot $library) -PathType Leaf)) {
    throw "Required vcpkg library '$library' is missing from '$libRoot'."
  }
}

$buildToolsRoot = if ($env:COREVIA_WINDOWS_BUILD_TOOLS_ROOT) {
  $env:COREVIA_WINDOWS_BUILD_TOOLS_ROOT
} else {
  'C:\BuildTools'
}
$msvcRoot = Join-Path $buildToolsRoot 'VC\Tools\MSVC'
$msvcInclude = Get-ChildItem -LiteralPath $msvcRoot -Directory -ErrorAction SilentlyContinue |
  Sort-Object Name -Descending |
  Select-Object -First 1 |
  ForEach-Object { Join-Path $_.FullName 'include' }
if (-not $msvcInclude -or -not (Test-Path -LiteralPath $msvcInclude -PathType Container)) {
  throw "MSVC headers were not found below '$msvcRoot'."
}

$kitsRoot = if ($env:COREVIA_WINDOWS_KITS_ROOT) {
  $env:COREVIA_WINDOWS_KITS_ROOT
} else {
  'C:\Program Files (x86)\Windows Kits\10\Include'
}
$kit = Get-ChildItem -LiteralPath $kitsRoot -Directory -ErrorAction SilentlyContinue |
  Sort-Object Name -Descending |
  Select-Object -First 1
if (-not $kit) {
  throw "Windows SDK headers were not found below '$kitsRoot'."
}
$kitInclude = $kit.FullName
foreach ($subdir in @('ucrt', 'shared', 'um')) {
  if (-not (Test-Path -LiteralPath (Join-Path $kitInclude $subdir) -PathType Container)) {
    throw "Windows SDK include directory '$subdir' is missing from '$kitInclude'."
  }
}

$env:VCPKG_ROOT = $vcpkgRoot
$env:VCPKG_INSTALLED_ROOT = $installedRoot
$env:BINDGEN_EXTRA_CLANG_ARGS = @(
  "-I`"$($msvcInclude -replace '\\', '/')`"",
  "-I`"$((Join-Path $kitInclude 'ucrt') -replace '\\', '/')`"",
  "-I`"$((Join-Path $kitInclude 'shared') -replace '\\', '/')`"",
  "-I`"$((Join-Path $kitInclude 'um') -replace '\\', '/')`""
) -join ' '

Write-Host "Windows native build environment: Rust $(rustc --version)"
Write-Host "vcpkg: $vcpkgRoot ($triplet)"
Write-Host "MSVC headers: $msvcInclude"
Write-Host "Windows SDK headers: $kitInclude"
Write-Host 'BINDGEN_EXTRA_CLANG_ARGS configured for the current CI process.'
