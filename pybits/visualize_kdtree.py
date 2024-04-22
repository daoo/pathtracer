#!/usr/bin/env python3

import json
import matplotlib.pyplot as plt
import numpy as np
import rerun


def read(path: str):
    with open(path) as f:
        return json.load(f)


def axis_number(axis: str) -> int:
    if axis == "X":
        return 0
    if axis == "Y":
        return 1
    if axis == "Z":
        return 2

    raise ValueError(f"Unknown axis {axis}.")


def visualize_kdnode(colormap, aabb, root):
    stack = [(0, aabb, root)]

    mins = []
    sizes = []
    depths = []
    while stack:
        (depth, aabb, node) = stack.pop()
        if isinstance(node, list):
            continue
        else:
            axis = axis_number(node["axis"])
            distance = node["distance"]

            plane = aabb.copy()
            plane[:, axis] = distance
            left_aabb = aabb.copy()
            left_aabb[1, axis] = distance
            right_aabb = aabb.copy()
            right_aabb[0, axis] = distance

            mins.append(plane[0])
            sizes.append(plane[1] - plane[0])
            depths.append(depth)

            stack.append((depth + 1, left_aabb, node["left"]))
            stack.append((depth + 1, right_aabb, node["right"]))

    rerun.log(
        "world/kdtree",
        rerun.Boxes3D(mins=mins, sizes=sizes, colors=colormap[depths]),
        timeless=True,
    )


def max_depth(node) -> int:
    if isinstance(node, list):
        return 0
    else:
        return 1 + max(max_depth(node["left"]), max_depth(node["right"]))


def node_count(node) -> int:
    if isinstance(node, list):
        return 1
    else:
        return 1 + node_count(node["left"]) + node_count(node["right"])


def visualize(kdtree):
    triangles = np.array(kdtree["triangles"])
    n = max_depth(kdtree["root"])
    colormap = plt.colormaps["plasma"].resampled(n)(np.linspace(0, 1, n))

    aabb_min = triangles.min(axis=0).min(axis=0)
    aabb_max = triangles.max(axis=0).max(axis=0)
    aabb = np.array([aabb_min, aabb_max])
    visualize_kdnode(colormap, aabb, kdtree["root"])
