#!/usr/bin/env bash

set -euo pipefail

if [[ "${COREVIA_DESK_PLATFORM:-}" != "macos" ]]; then
  printf '%s\n' 'The macOS native environment gate must run on a macOS release job.' >&2
  exit 1
fi

vcpkg_root="${COREVIA_MACOS_VCPKG_ROOT:-/Volumes/Personal/Projects/corevia/corevia-desk/vcpkg}"
installed_root="${COREVIA_MACOS_VCPKG_INSTALLED_ROOT:-${vcpkg_root}/installed}"
triplet="${COREVIA_MACOS_VCPKG_TRIPLET:-arm64-osx}"
baseline="${COREVIA_MACOS_VCPKG_BASELINE:-9e593bb18ea69cc5095e012465dcd675a822ed0d}"

if [[ "${COREVIA_DESK_ARCH:-}" != "arm64" ]]; then
  printf '%s\n' 'The macOS native environment gate only supports the declared arm64 target.' >&2
  exit 1
fi

vcpkg="${vcpkg_root}/vcpkg"
if [[ ! -x "$vcpkg" ]]; then
  printf 'The declared vcpkg executable is missing: %s\n' "$vcpkg" >&2
  exit 1
fi

if [[ -d "${vcpkg_root}/.git" ]]; then
  actual_baseline="$(git -C "$vcpkg_root" rev-parse HEAD)"
  if [[ "$actual_baseline" != "$baseline" ]]; then
    printf 'The declared vcpkg baseline does not match: expected %s, got %s\n' "$baseline" "$actual_baseline" >&2
    exit 1
  fi
fi

if [[ ! -d "${installed_root}/${triplet}" ]]; then
  printf 'The declared vcpkg install root is missing the target triplet: %s\n' "${installed_root}/${triplet}" >&2
  exit 1
fi

for library in libaom.a libopus.a libvpx.a libyuv.a; do
  if [[ ! -f "${installed_root}/${triplet}/lib/${library}" ]]; then
    printf 'Required native library is missing: %s\n' "${installed_root}/${triplet}/lib/${library}" >&2
    exit 1
  fi
done

for header in aom/aom_codec.h opus/opus.h vpx/vpx_codec.h libyuv.h; do
  if [[ ! -f "${installed_root}/${triplet}/include/${header}" ]]; then
    printf 'Required native header is missing: %s\n' "${installed_root}/${triplet}/include/${header}" >&2
    exit 1
  fi
done

if ! xcrun --find clang >/dev/null 2>&1; then
  printf '%s\n' 'The active Xcode command-line toolchain is missing clang.' >&2
  exit 1
fi

export VCPKG_ROOT="$vcpkg_root"
export VCPKG_INSTALLED_ROOT="$installed_root"
export VCPKG_DEFAULT_TRIPLET="$triplet"

printf 'macOS native environment ready: vcpkg=%s triplet=%s baseline=%s\n' "$VCPKG_ROOT" "$VCPKG_DEFAULT_TRIPLET" "$baseline"
