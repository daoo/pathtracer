#!/usr/bin/env bash

set -eu

for file in "$@"; do
  diff -s <(./out_clang_debug/print-tree-sah "$file" 2> /dev/null) <(./out_clang_debug/print-tree-sah_fast "$file" 2> /dev/null)
done
