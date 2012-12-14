target = debug

path      = $(realpath .)
build_dir = $(path)/build/$(target)
front_dir = $(build_dir)/src/frontends

build:
	+@make --no-print-directory -C $(build_dir) all

clean:
	+@make --no-print-directory -C $(build_dir) clean

debug:
	gdb -ex "set args $(path)/scenes/cube.obj $(path)/build/" \
	      $(front_dir)/gui/schwar-gl

run:
	$(front_dir)/gui/schwar-gl \
	       $(path)/scenes/cube.obj \
	       $(path)/build/

cmake:
	mkdir -p $(build_dir)
	cd $(build_dir); cmake -DCMAKE_BUILD_TYPE=$(target) $(path)

.PHONY: build clean cmake links debug run
