#!/usr/bin/env bash

dir="$1"
repo="$2"
model="$3"
threads=$4
width=$5
height=$6
samples=$7

mkdir -p "$dir"
cd "$dir"

CXX=g++ CXXFLAGS="-Ofast -flto -mtune=native -march=native -DNDEBUG" make target=dist cmake || exit 1
make target=dist build || exit 1

./build/dist/frontends/commandline/pathtracer \
  -m "$model" \
  -o "$dir/build/" \
  -t $threads \
  -x $width \
  -y $height \
  -s $samples
