path = $(realpath .)
target = debug

build:
	+@make --no-print-directory -C build/$(target) all

clean:
	+@make --no-print-directory -C build/$(target) clean

debug:
	cgdb -- -ex "set args -m $(path)/scenes/cube.obj -o /tmp" \
		build/debug/src/frontends/gui/pathtracer-gl

run:
	build/release_clang/src/frontends/gui/pathtracer-gl \
		-m $(path)/scenes/cube.obj \
		-o /tmp

cmake_debug:
	@mkdir -p build/debug
	@cd build/debug; \
		CC=clang CXX=clang CXXOPTS="-g -Og" cmake -DCMAKE_BUILD_TYPE=debug $(path)

cmake_release_clang:
	@mkdir -p build/release_clang
	@cd build/release_clang; \
		CC=clang CXX=clang CXXOPTS="-Ofast" cmake -DCMAKE_BUILD_TYPE=release $(path)

cmake_release_gcc:
	@mkdir -p build/release_gcc
	@cd build/release_gcc; \
		CC=gcc CXX=g++ CXXOPTS="-Ofast -march=native" cmake -DCMAKE_BUILD_TYPE=release $(path)

distclean:
	rm -rf build/debug build/release_clang

.PHONY: build clean cmake links debug run
