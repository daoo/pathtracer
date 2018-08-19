#!/usr/bin/env bash

list-source() {
  find \
    geometry kdtree pathtracer pathtracer-gl tests trace util wavefront \
    -type f -and -not -name catch.h
}

format-check() {
  diff -u <(cat "$@") <(clang-format "$@")
}

call-cpplint() {
  cpplint \
    --quiet \
    --filter=-legal/copyright,-build/c++11,-readability/todo \
    "$@"
}

call-cppcheck() {
  cppcheck \
    --language=c++ \
    --std=c++11 \
    --enable=all \
    --quiet \
    "$@"
}

files="$(list-source)"
{ call-cpplint $files > /dev/null; } 2>&1 | grep -v '#include <algorithm>'
call-cppcheck $files
format-check $files
