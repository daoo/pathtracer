#!/usr/bin/env python

import sys


def right_triangle(x, y, size):
    return ((x, y), (x + size, y), (x, y + size))


def obj(triangles):
    print("mtllib cube.mtl")
    for (p0, p1, p2) in triangles:
        print("v {0} {1} 0".format(*p0))
        print("v {0} {1} 0".format(*p1))
        print("v {0} {1} 0".format(*p2))
    for i in range(0, len(triangles)):
        print("vn 0 0 1")
    print("usemtl White")
    for i in range(0, len(triangles)):
        print("f {v0}//{n} {v1}//{n} {v2}//{n}".format(
            v0=i * 3 + 1,
            v1=i * 3 + 2,
            v2=i * 3 + 3,
            n=i + 1,
        ))


def gen_triangle_grid(xcount, ycount, size, offset):
    for i in range(0, xcount):
        for j in range(0, ycount):
            x = i * (size + offset)
            y = j * (size + offset)
            yield right_triangle(x, y, size)


xcount = int(sys.argv[1])
ycount = int(sys.argv[2])

obj(list(gen_triangle_grid(xcount, ycount, 100, 50)))
