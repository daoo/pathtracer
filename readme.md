# Pathtracer

[![Build Status](https://travis-ci.org/daoo/pathtracer.svg?branch=master)](https://travis-ci.org/daoo/pathtracer)

A pathtracer written in C++ which loads and renders obj files with (custom) mtl files.

![Render](https://raw.github.com/daoo/pathtracer/master/resources/cornell_1080x1080_2048.png)

## Building

Required libraries: FreeImage, GLM, GLEW, GLUT, OpenGL.

Uses meson and ninja for building:

    meson builddir
    ninja -C builddir

The [Dockerfile](https://github.com/daoo/pathtracer/blob/master/Dockerfile)
describes the build environment that is used with travis.

## Running

Example commands:

    ./builddir/pathtracer scenes/cornell.obj scenes/cornell.mtl /tmp/cornell.png 1000 1000 128 8
    ./builddir/pathtracer-gl scenes/cornell.obj scenes/cornell.mtl /tmp
