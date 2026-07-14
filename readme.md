# Pathtracer

A pathtracer written in rust which loads and renders obj files with (custom) mtl files.

![Render](https://raw.github.com/daoo/pathtracer/master/resources/cornell_1080x1080_2048.png)

## Building

    cargo build --release

## Running

Example command:

    ./target/release/pathtracer -i resources/cornell.obj -o /tmp/cornell.png -w 1000 -h 1000 -n 128 -t 12
