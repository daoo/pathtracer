#!/usr/bin/env python3

import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
import rerun

RAY_DTYPE = np.dtype(
    [
        ("i", np.uint16),
        ("px", np.uint16),
        ("py", np.uint16),
        ("inf", np.uint8),
        ("ox", np.float32),
        ("oy", np.float32),
        ("oz", np.float32),
        ("dx", np.float32),
        ("dy", np.float32),
        ("dz", np.float32),
    ]
)


def read(path):
    return pd.DataFrame(np.fromfile(path, RAY_DTYPE))


def visualize_as_one_entity(rays):
    rays["tx"] = rays.ox + rays.dx + rays.inf * rays.dx * 10.0
    rays["ty"] = rays.oy + rays.dy + rays.inf * rays.dy * 10.0
    rays["tz"] = rays.oz + rays.dz + rays.inf * rays.dz * 10.0
    colors = plt.colormaps["viridis"](np.linspace(0.0, 1.0, len(rays)))

    segments = rays[["ox", "oy", "oz", "tx", "ty", "tz"]].to_numpy()
    segments = segments.reshape((len(rays), 2, 3))
    rerun.log(
        "world/rays",
        rerun.LineStrips3D(segments, radii=0.001, colors=colors),
        timeless=True,
    )


def visualize_grouped_per_path(rays):
    rays["tx"] = rays.ox + rays.dx + rays.inf * rays.dx * 10.0
    rays["ty"] = rays.oy + rays.dy + rays.inf * rays.dy * 10.0
    rays["tz"] = rays.oz + rays.dz + rays.inf * rays.dz * 10.0
    paths = rays.groupby(["i", "px", "py"])
    colors = plt.colormaps["viridis"](np.linspace(0.0, 1.0, len(paths)))

    for ((i, px, py), path) in paths:
        segments = path[["ox", "oy", "oz", "tx", "ty", "tz"]].values
        segments = segments.reshape((len(path), 2, 3))
        rerun.log(
            f"world/rays/ray_{i}_{px}x{py}",
            rerun.LineStrips3D(segments, radii=0.001, colors=colors),
            timeless=True,
        )
