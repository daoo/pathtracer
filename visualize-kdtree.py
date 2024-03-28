#!/usr/bin/env python3

import argparse
import json
import matplotlib.pyplot as plt
import numpy as np
import rerun
import sys


def loop_around(points):
    return np.concatenate(
        (points, np.array([points[:, 0]]).reshape((len(points), 1, 3))),
        axis=1)


def read_kdtree(path):
    with open(path) as f:
        return json.load(f)


def visualize_triangles(triangles):
    rerun.log(
        'world/triangles',
        rerun.LineStrips3D(
            loop_around(triangles),
            radii=0.002,
            colors=[(255, 255, 255)],),
        timeless=True)


def axis_number(axis):
    if axis == "X":
        return 0
    if axis == "Y":
        return 1
    if axis == "Z":
        return 2


def visualize_kdnode(color, depth, path, parent, node):
    if type(node) is list:
        return
    else:
        axis = axis_number(node["axis"])
        distance = node["distance"]

        plane = parent.copy()
        plane[:, axis] = distance
        left_aabb = parent.copy()
        left_aabb[1, axis] = distance
        right_aabb = parent.copy()
        right_aabb[0, axis] = distance

        # rerun.log(f'world/{path}/{node["axis"]}={distance}', rerun.Boxes3D(
        #     mins=[plane[0]],
        #     sizes=[plane[1] - plane[0]]), timeless=True)
        rerun.log(f'world/{path}/aabb', rerun.Boxes3D(
            mins=[parent[0]],
            sizes=[parent[1] - parent[0]],
            radii=0.001,
            colors=color(depth)), timeless=True)

        visualize_kdnode(color, depth + 1, f'{path}/l', left_aabb, node["left"])
        visualize_kdnode(color, depth + 1, f'{path}/r', right_aabb, node["right"])


def max_depth(node):
    if type(node) is list:
        return 0
    else:
        return 1 + max(max_depth(node['left']), max_depth(node['right']))


def visualize(kdtree):
    rerun.init('kdtree_build')
    rerun.connect()
    rerun.log('world', rerun.ViewCoordinates.RIGHT_HAND_Y_UP, timeless=True)

    triangles = np.array(kdtree["triangles"])
    visualize_triangles(triangles)

    n = max_depth(kdtree['root'])
    colormap = plt.colormaps['plasma'].resampled(n)

    def color(depth):
        return colormap(depth / float(n))

    parent_min = triangles.min(axis=0).min(axis=1)
    parent_max = triangles.max(axis=0).max(axis=1)
    parent = np.array([parent_min, parent_max])
    visualize_kdnode(color, 0, 'root', parent, kdtree["root"])


def program(path):
    print(f'Reading "{path}"...')
    kdtree = read_kdtree(path)
    visualize(kdtree)


def main():
    parser = argparse.ArgumentParser(
        description="Visualize kdtree with rerun.",
        formatter_class=argparse.ArgumentDefaultsHelpFormatter)
    parser.add_argument("path", help="kdtree.json file path")
    args = parser.parse_args()
    sys.exit(program(args.path))


main()
