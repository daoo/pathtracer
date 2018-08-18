#!/usr/bin/env python

import argparse


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


parser = argparse.ArgumentParser(
    description="Generate 2D grid of triangles.",
    formatter_class=argparse.ArgumentDefaultsHelpFormatter)

parser.add_argument(
    "xcount",
    type=int,
    help="number of triangles in x-direction")

parser.add_argument(
    "ycount",
    type=int,
    help="number of triangles in x-direction")

parser.add_argument(
    "-s", "--size",
    type=int,
    default=100,
    help="triangle size in x and y")

parser.add_argument(
    "-o", "--offset",
    type=int,
    default=50,
    help="grid offset in x and y")

args = parser.parse_args()

obj(list(gen_triangle_grid(args.xcount, args.ycount, args.size, args.offset)))
