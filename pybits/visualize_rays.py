#!/usr/bin/env python3

import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
import rerun

RAY_DTYPE = np.dtype(
    [
        ("iteration", np.uint16),
        ("px", np.uint16),
        ("py", np.uint16),
        ("bounces", np.uint8),
        ("shadow", np.uint8),
        ("intersect", np.uint8),
        ("ox", np.float32),
        ("oy", np.float32),
        ("oz", np.float32),
        ("dx", np.float32),
        ("dy", np.float32),
        ("dz", np.float32),
    ]
)


def read(path):
    rays = pd.DataFrame(np.fromfile(path, RAY_DTYPE))
    rays["tx"] = rays.ox + rays.dx
    rays["ty"] = rays.oy + rays.dy
    rays["tz"] = rays.oz + rays.dz
    return rays


def visualize_as_one_entity(rays):
    def plot(name, rays, color):
        segments = rays[["ox", "oy", "oz", "tx", "ty", "tz"]].to_numpy()
        segments = segments.reshape((len(rays), 2, 3))
        rerun.log(
            f"world/rays/{name}",
            rerun.LineStrips3D(segments, radii=0.001, colors=color),
            static=True,
        )

    rays_environment = rays[(rays.shadow == 0) & (rays.intersect == 0)]
    rays_surface = rays[(rays.shadow == 0) & (rays.intersect == 1)]
    rays_light = rays[(rays.shadow == 1) & (rays.intersect == 0)]
    rays_shadow = rays[(rays.shadow == 1) & (rays.intersect == 1)]
    plot("environment", rays_environment, [255, 0, 0])
    plot("surface", rays_surface, [0, 255, 0])
    plot("light", rays_light, [255, 255, 0])
    plot("shadow", rays_shadow, [0, 255, 255])


def visualize_grouped_per_path(rays):
    colormap = plt.colormaps["viridis"].resampled(rays.bounces.max())
    rays["color"] = rays.bounces / rays.bounces.max()
    paths = rays.groupby(["iteration", "px", "py"])

    def plot(name, path):
        segments = path[["ox", "oy", "oz", "tx", "ty", "tz"]].values
        segments = segments.reshape((len(path), 2, 3))
        rerun.log(
            f"world/rays/{name}",
            rerun.LineStrips3D(segments, radii=0.001, colors=colormap(path.color)),
            static=True,
        )

    for (iteration, px, py), path in paths:
        path_environment = path[(path.shadow == 0) & (path.intersect == 0)]
        path_surface = path[(path.shadow == 0) & (path.intersect == 1)]
        path_light = path[(path.shadow == 1) & (path.intersect == 0)]
        path_shadow = path[(path.shadow == 1) & (path.intersect == 1)]
        plot(f"{iteration}_{px}x{py}/environment", path_environment)
        plot(f"{iteration}_{px}x{py}/surface", path_surface)
        plot(f"{iteration}_{px}x{py}/light", path_light)
        plot(f"{iteration}_{px}x{py}/shadow", path_shadow)
