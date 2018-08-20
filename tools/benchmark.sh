#!/usr/bin/env bash

set -eu

ninja -C out_gcc_release pathtracer
./out_gcc_release/pathtracer scenes/cornell.obj scenes/cornell.mtl /dev/null 512 512 16 4
