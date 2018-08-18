#!/usr/bin/env bash

set -eu

rg --files --type cpp | entr -c sh -c "ninja -C out_clang_debug && ./out_clang_debug/unit-tests"
