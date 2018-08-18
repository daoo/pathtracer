#!/usr/bin/env bash

set -eu

if [[ $# != 1 ]]; then
  echo "Usage: $0 MODEL.OBJ" 1>&2
  exit 1
fi

model="$1"

for impl in sah sah_fast; do
  ./out_clang_debug/print-tree-svg-$impl "$model" > /tmp/$impl.svg
  inkscape --without-gui --export-png /tmp/$impl.png -D /tmp/$impl.svg
  rm /tmp/$impl.svg
done
