import json
import numpy as np
import rerun


def loop_around(points):
    return np.concatenate(
        (points, np.array([points[:, 0]]).reshape((len(points), 1, 3))), axis=1
    )


def read(path):
    with open(path) as f:
        return np.array(json.load(f)["triangles"])


def visualize(triangles):
    rerun.log(
        "world/triangles",
        rerun.LineStrips3D(
            loop_around(triangles),
            radii=0.002,
            colors=[(255, 255, 255)],
        ),
        timeless=True,
    )
