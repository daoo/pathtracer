#!/usr/bin/env python3

import numpy as np
import rerun


def loop_around(points):
    return np.concatenate(
            (points, np.array([points[:, 0]]).reshape((len(points), 1, 3))),
            axis=1)


parent = np.array([[-0.44965002, 0.5579, 0.0], [0.55035, 0.4421, 1.0]])
plane = np.array([0.0, 0.0, -0.09244999])
in_left = np.array([[[-1.0, 1.0, -1.0], [-1.0, 1.0, 1.0], [-1.0, -1.0, -1.0]], [[1.0, 1.0, 1.0], [-1.0, 1.0, 1.0], [1.0, 1.0, -1.0]], [[-1.0, 1.0, -1.0], [1.0, 1.0, -1.0], [-1.0, 1.0, 1.0]]])
in_right = np.array([[[-1.0, -1.0, 1.0], [-1.0, -1.0, -1.0], [-1.0, 1.0, 1.0]], [[-1.0, 1.0, -1.0], [-1.0, 1.0, 1.0], [-1.0, -1.0, -1.0]], [[1.0, 1.0, 1.0], [-1.0, 1.0, 1.0], [1.0, 1.0, -1.0]], [[-1.0, 1.0, -1.0], [1.0, 1.0, -1.0], [-1.0, 1.0, 1.0]]])
in_neither = np.array([[[-1.0, -1.0, -1.0], [1.0, -1.0, -1.0], [-1.0, 1.0, -1.0]], [[1.0, 1.0, -1.0], [-1.0, 1.0, -1.0], [1.0, -1.0, -1.0]]])

rerun.init('kdtree_build')
rerun.connect()
rerun.log('world', rerun.ViewCoordinates.RIGHT_HAND_Y_UP, timeless=True)

rerun.log('world/parent', rerun.Boxes3D(
    centers=[parent[0]],
    half_sizes=[parent[1]]))
rerun.log('world/plane', rerun.Boxes3D(
    centers=[[parent[0][0], parent[0][1], plane[0]]],
    half_sizes=[parent[1][0], parent[1][1], 0.0]))


rerun.log('world/left', rerun.LineStrips3D(
    loop_around(in_left), radii=0.001, colors=[(0, 255, 0)],), timeless=True)

rerun.log('world/right', rerun.LineStrips3D(
    loop_around(in_right), radii=0.001, colors=([0, 0, 255])), timeless=True)

rerun.log('world/lost', rerun.LineStrips3D(
    loop_around(in_neither), radii=0.001, colors=[255, 0, 0]), timeless=True)
