# Pathtracer

[![build](https://github.com/daoo/pathtracer/workflows/build/badge.svg)](https://github.com/daoo/pathtracer/actions?query=workflow%3Abuild)

A pathtracer written in rust which loads and renders obj files with (custom) mtl files.

![Render](https://raw.github.com/daoo/pathtracer/master/resources/cornell_1080x1080_2048.png)

## Building

    cargo build --release

The [Dockerfile](https://github.com/daoo/pathtracer/blob/master/Dockerfile)
describes the build environment that is used with github workflows.

## Running

Example command:

    ./target/release/pathtracer -i scenes/cornell.obj -o /tmp/cornell.png -w 1000 -h 1000 -n 128 -t 12
