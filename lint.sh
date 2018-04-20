#!/usr/bin/env bash

format-check() {
  diff -u <(cat $@) <(clang-format $@)
}

files="$(find geometry kdtree pathtracer pathtracer-gl tests trace util wavefront -type f -and -not -name catch.h)"

cpplint --quiet --filter=-legal/copyright,-build/c++11,-readability/todo --quiet $files 2>&1 >/dev/null | grep -v '#include <algorithm>'

format-check $files
