#!/usr/bin/env bash

set -eu

list-source() {
  find \
    . \
    -type f \
    -and \( -name '*.cc' -or -name '*.h' \) \
    -and -not -path './third_party/*' \
    -and -not -path './out_*' \
    "$@"
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
    -j 4 \
    --language=c++ \
    --std=c++11 \
    --enable=style \
    --quiet \
    -I"." \
    "$@"
}

source_all="$(list-source)"
source_nounittest="$(list-source -not -name '*unittest.cc' -and -not -name 'unit-tests.cc')"

call-cpplint $source_all 2>&1 >/dev/null

# cppcheck doesn't handle catch.h very well (to many ifdefs)
call-cppcheck $source_nounittest

format-check $source_all
