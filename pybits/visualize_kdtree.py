#!/usr/bin/env python3

import json
import matplotlib.pyplot as plt
import numpy as np
import rerun


def read(path):
    with open(path) as f:
        return json.load(f)


def axis_number(axis):
    if axis == 'X':
        return 0
    if axis == 'Y':
        return 1
    if axis == 'Z':
        return 2


def visualize_kdnode(color, depth, path, aabb, node):
    if type(node) is list:
        return
    else:
        axis = axis_number(node['axis'])
        distance = node['distance']

        plane = aabb.copy()
        plane[:, axis] = distance
        left_aabb = aabb.copy()
        left_aabb[1, axis] = distance
        right_aabb = aabb.copy()
        right_aabb[0, axis] = distance

        # rerun.log(f'world/{path}/{node['axis']}={distance}', rerun.Boxes3D(
        #     mins=[plane[0]],
        #     sizes=[plane[1] - plane[0]]), timeless=True)
        rerun.log(f'world/{path}/aabb', rerun.Boxes3D(
            mins=[aabb[0]],
            sizes=[aabb[1] - aabb[0]],
            radii=0.001,
            colors=color(depth)), timeless=True)

        visualize_kdnode(
            color, depth + 1, f'{path}/l', left_aabb, node['left'])
        visualize_kdnode(
            color, depth + 1, f'{path}/r', right_aabb, node['right'])


def max_depth(node):
    if type(node) is list:
        return 0
    else:
        return 1 + max(max_depth(node['left']), max_depth(node['right']))


def node_count(node):
    if type(node) is list:
        return 1
    else:
        return 1 + max_depth(node['left']) + max_depth(node['right'])


def visualize(kdtree):
    triangles = np.array(kdtree['triangles'])
    n = max_depth(kdtree['root'])
    colormap = plt.colormaps['plasma'].resampled(n)

    def color(depth):
        return colormap(depth / float(n))

    aabb_min = triangles.min(axis=0).min(axis=1)
    aabb_max = triangles.max(axis=0).max(axis=1)
    aabb = np.array([aabb_min, aabb_max])
    visualize_kdnode(color, 0, 'root', aabb, kdtree['root'])
