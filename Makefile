default: clang-debug

all: clang-debug linux-release linux-extreme

build/%.ninja: build.ninja.template configure
	./configure

clang-debug: build/clang-debug.ninja
	@ninja -C build -f clang-debug.ninja

gcc-debug: build/gcc-debug.ninja
	@ninja -C build -f gcc-debug.ninja

linux-release: build/linux-release.ninja
	@ninja -C build -f linux-release.ninja

linux-extreme: build/linux-extreme.ninja
	@ninja -C build -f linux-extreme.ninja

distclean:
	rm -fr build
