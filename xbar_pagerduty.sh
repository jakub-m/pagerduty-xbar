#!/bin/bash
set -eu

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)

config_file=""
if [[ -e "$PWD/.pg_xbar.conf" ]]; then
  config_file="$PWD/.pg_xbar.conf"
fi

if [[ -f "$HOME/.config/pg_xbar.conf" ]]; then
  config_file="$HOME/.config/pg_xbar.conf"
fi

if [[ -z "${config_file}" ]]; then
  echo "config file missing: $HOME/.config/pg_xbar.conf"
  exit 1
fi

>&2 echo "config_file: ${config_file}"

set -a
source "${config_file}"
set +a
bin="pagerduty-xbar"
if [[ -e Cargo.toml ]]; then
  cargo run -- "$@"
else
  release_file="${SCRIPT_DIR}/target/release/${bin}"
  if [[ -e "${release_file}" ]]; then
    exec "${release_file}"
  elif [[ -e "${SCRIPT_DIR}/${bin}" ]]; then
    # This case is for homebrew install when the bash script lands in the same bin/ directory with the binary.
    exec "${SCRIPT_DIR}/${bin}"
  else
    echo "⚠️"
    echo "---"
    echo 'target/release/pagerduty-xbar missing. Make sure to run `cargo build --release` or that the binary is in the same directory as the script.'
  fi
fi
