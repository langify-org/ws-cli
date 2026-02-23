#!/usr/bin/env bash
set -euo pipefail

FILE_PATH=$(jq -r '.tool_input.file_path')

if [[ "$FILE_PATH" == *.rs ]]; then
  cargo fmt 2>/dev/null || true
fi
