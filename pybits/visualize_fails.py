#!/usr/bin/env python3

import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
import rerun

RAY_DTYPE = np.dtype(
    [
        ("i", np.uint16),
        ("ox", np.float32),
        ("oy", np.float32),
        ("oz", np.float32),
        ("dx", np.float32),
        ("dy", np.float32),
        ("dz", np.float32),
        ("cx", np.float32),
        ("cy", np.float32),
        ("cz", np.float32),
        ("ax", np.float32),
        ("ay", np.float32),
        ("az", np.float32),
    ]
)


def read(path):
    return pd.DataFrame(np.fromfile(path, RAY_DTYPE))


def visualize(rays):
    rays["tx"] = rays.ox + rays.dx
    rays["ty"] = rays.oy + rays.dy
    rays["tz"] = rays.oz + rays.dz

    color_count = len(rays)
    colormap = plt.colormaps["viridis"].resampled(color_count)

    def color(path_number):
        return colormap(path_number / float(color_count))

    for ray in rays.itertuples():
        segments = np.array([[[ray.ox, ray.oy, ray.oz], [ray.tx, ray.ty, ray.tz]]])
        correct_point = np.array([ray.cx, ray.cy, ray.cz])
        actual_point = np.array([ray.ax, ray.ay, ray.az])
        path = f"world/ray{ray.i}"
        rerun.log(
            f"{path}/ray",
            rerun.LineStrips3D(segments, radii=0.001, colors=color(ray.i)),
            static=True,
        )
        rerun.log(
            f"{path}/points",
            rerun.Points3D(
                [correct_point, actual_point],
                radii=0.02,
                colors=[(0, 255, 0), (255, 0, 0)],
            ),
            static=True,
        )
