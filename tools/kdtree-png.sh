#!/usr/bin/env bash

set -eu

if [[ $# != 1 ]]; then
  echo "Usage: $0 MODEL.OBJ" 1>&2
  exit 1
fi

model="$1"

./out_clang_debug/print-tree-svg-sah "$model" > /tmp/kdtree.svg
inkscape --without-gui --export-png /tmp/kdtree.png -D /tmp/kdtree.svg
rm /tmp/kdtree.svg
