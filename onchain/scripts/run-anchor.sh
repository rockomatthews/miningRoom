#!/usr/bin/env bash
set -euo pipefail

resolve_anchor() {
  if [[ -n "${ANCHOR_BIN:-}" ]] && command -v "${ANCHOR_BIN}" >/dev/null 2>&1; then
    echo "${ANCHOR_BIN}"
    return
  fi

  if [[ -x "${HOME}/.cargo/bin/anchor" ]]; then
    echo "${HOME}/.cargo/bin/anchor"
    return
  fi

  if command -v anchor >/dev/null 2>&1; then
    echo "$(command -v anchor)"
    return
  fi

  echo "Anchor CLI not found. Install via 'cargo install --git https://github.com/coral-xyz/anchor anchor-cli --tag v0.30.1' or set ANCHOR_BIN." >&2
  exit 1
}

ANCHOR_BIN_PATH="$(resolve_anchor)"
exec env RUSTUP_TOOLCHAIN=nightly RUSTC_BOOTSTRAP=1 RUSTFLAGS="--cfg=proc_macro_span --cfg=procmacro2_semver_exempt --cfg=nightly" "${ANCHOR_BIN_PATH}" "$@"

