#!/usr/bin/env bash

dir="$1"
repo="$2"

if [[ -d "$dir/.git" ]]; then
  cd "$dir"
  git pull || exit 1

  # TODO: Only compile if there are changes
  make target=dist build || exit 3
else
  git clone "$repo" "$dir" || exit 1
  cd "$dir"

  make target=dist cmake || exit 2
  make target=dist build || exit 3
fi

./build/dist/frontends/commandline/pathtracer -m
