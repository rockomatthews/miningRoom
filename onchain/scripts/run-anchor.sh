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
exec "${ANCHOR_BIN_PATH}" "$@"

