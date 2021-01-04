#!/usr/bin/env bash

DOTFILES_COMMIT_HASH="e517c25"

dot::clone() {
   git clone "https://github.com/denisidoro/dotfiles.git" "$DOTFILES"
   cd "$DOTFILES" && git checkout "$DOTFILES_COMMIT_HASH"
}

dot::install_if_necessary() {
   [ -n "${DOTFILES:-}" ] && return
   export DOTFILES="${ABRA_HOME}/dotfiles"
   export PATH="${DOTFILES}/bin:${PATH}"
   $(dot::clone 2>/dev/null || true)
}

export ABRA_HOME="${ABRA_HOME:-$(cd "$(dirname "$0")/.." && pwd)}"

dot::install_if_necessary
source "${DOTFILES}/scripts/core/main.sh"
source "${DOTFILES}/scripts/core/log.sh"

export ABRA_BIN="${ABRA_HOME}/target/release/abra"
[ -f "$ABRA_BIN" ] || export ABRA_BIN="${ABRA_HOME}/target/debug/abra"
[ -f "$ABRA_BIN" ] || export ABRA_BIN="${ABRA_HOME}/scripts/run"