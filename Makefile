default: linux-debug

all: linux-debug linux-release linux-extreme

build/%.ninja: build.ninja.template configure
	./configure

linux-debug: build/linux-debug.ninja
	@ninja -C build -f linux-debug.ninja

linux-release: build/linux-release.ninja
	@ninja -C build -f linux-release.ninja

linux-extreme: build/linux-extreme.ninja
	@ninja -C build -f linux-extreme.ninja

distclean:
	rm -fr build
