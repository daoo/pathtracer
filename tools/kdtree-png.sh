#!/usr/bin/env bash

set -eu

dir="out_clang_debug"
inp="$1"

./$dir/print-tree-svg-sah "$inp" > /tmp/test.svg
inkscape --without-gui --export-png /tmp/test.png -D /tmp/test.svg
rm /tmp/test.svg
