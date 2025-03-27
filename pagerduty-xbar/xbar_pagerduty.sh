#!/bin/bash
set -eu

config_file=""
if [[ -e "$PWD/.pg_xbar.conf" ]]; then
  config_file=""$PWD/.pg_xbar.conf""
fi

if [[ -f "$HOME/.config/pg_xbar.conf" ]]; then
  config_file="$HOME/.config/pg_xbar.conf"
fi

if [[ -z "${config_file}" ]]; then
  echo "config file missing: $HOME/.config/pg_xbar.conf"
  exit 1
fi

2>echo "config_file: ${config_file}"

set -a
source "${config_file}"
set +a
cargo run | tee response.json
