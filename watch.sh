#!/usr/bin/env bash

watchexec --exts meson,cc,h "ninja -C out_clang_debug test"
