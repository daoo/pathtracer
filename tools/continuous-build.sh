#!/usr/bin/env bash

set -eu

rg --files --type cpp | entr ninja -C "${1:-out_clang_debug}"
