target = debug

path      = $(realpath .)
build_dir = $(path)/build/$(target)

build:
	+@make -j2 --no-print-directory -C $(build_dir) all

clean:
	+@make --no-print-directory -C $(build_dir) clean

links:
	ln -fsn $(path)/src/gui/data $(build_dir)/src/data
	ln -fsn $(path)/scenes $(build_dir)/src/scenes

debug: links
	cd $(build_dir)/src; gdb $(build_dir)/src/pathtracer-gl

run: links
	cd $(build_dir)/src; $(build_dir)/src/pathtracer-gl

cmake:
	mkdir -p $(build_dir)
	cd $(build_dir); cmake -DCMAKE_BUILD_TYPE=$(target) $(path)

.PHONY: build clean cmake links debug run
