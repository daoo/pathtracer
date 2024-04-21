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


def visualize(rays, color_segment):
    rays["tx"] = rays.ox + rays.dx + rays.inf * rays.dx * 10.0
    rays["ty"] = rays.oy + rays.dy + rays.inf * rays.dy * 10.0
    rays["tz"] = rays.oz + rays.dz + rays.inf * rays.dz * 10.0
    grouped = rays.groupby(["i", "px", "py"])
    color_count = grouped.cumcount().max() if color_segment else len(grouped)
    colormap = plt.colormaps["viridis"].resampled(color_count)

    def color(path_number, segment_count):
        if color_segment:
            return [colormap(i / float(color_count)) for i in range(segment_count)]
        else:
            return colormap(path_number / float(color_count))

    for n, (_, path) in zip(grouped.ngroup(), grouped):
        segments = path[["ox", "oy", "oz", "tx", "ty", "tz"]].to_numpy()
        segments = segments.reshape((len(path), 2, 3))
        iter = f"iter{path.i.iloc[0]}"
        pixel = f"{path.px.iloc[0]}x{path.py.iloc[0]}"
        path = f"world/rays/{iter}/{pixel}"
        rerun.log(
            path,
            rerun.LineStrips3D(segments, radii=0.001, colors=color(n, len(segments))),
            timeless=True,
        )
