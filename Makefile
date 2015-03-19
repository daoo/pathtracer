path = $(realpath .)
target = debug_gcc

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

cmake:
	@mkdir -p build/debug
	@mkdir -p build/debug_gcc
	@mkdir -p build/fast_clang
	@mkdir -p build/fast_gcc
	@mkdir -p build/size_gcc
	@cd build/debug; \
		CC=clang CXX=clang CXXOPTS="-stdlib=libc++ -lc++abi -g -Og" cmake -DCMAKE_BUILD_TYPE=debug $(path)
	@cd build/debug_gcc; \
		CC=gcc CXX=g++ CXXOPTS="-g -Og" cmake -DCMAKE_BUILD_TYPE=debug $(path)
	@cd build/fast_clang; \
		CC=clang CXX=clang CXXOPTS="-Ofast" cmake -DCMAKE_BUILD_TYPE=release $(path)
	@cd build/fast_gcc; \
		CC=gcc CXX=g++ CXXOPTS="-Ofast -mopt=native -march=native" cmake -DCMAKE_BUILD_TYPE=release $(path)
	@cd build/size_gcc; \
		CC=gcc CXX=g++ CXXOPTS="-Os" cmake -DCMAKE_BUILD_TYPE=release $(path)

distclean:
	rm -rf build/*

.PHONY: build clean cmake links debug run
