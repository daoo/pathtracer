#!/usr/bin/env python3

import argparse
import numpy as np
import pandas as pd
import rerun
import sys

RAY_DTYPE = np.dtype([
    ('i', np.uint16),
    ('px', np.uint16),
    ('py', np.uint16),
    ('ax', np.float32),
    ('ay', np.float32),
    ('az', np.float32),
    ('bx', np.float32),
    ('by', np.float32),
    ('bz', np.float32),
])


def read_rays(path):
    return pd.DataFrame(np.fromfile(path, RAY_DTYPE))


def visualize(rays):
    rerun.init('raytracing')
    rerun.connect()
    rerun.log('rays', rerun.ViewCoordinates.RIGHT_HAND_Y_UP, timeless=True)

    for (_, ray) in rays.groupby(['i', 'px', 'py']):
        segments = ray[['ax', 'ay', 'az', 'bx', 'by', 'bz']].to_numpy()
        segments = segments.reshape((len(ray), 2, 3))
        path = f'rays/iter{ray.i.iloc[0]}/{ray.px.iloc[0]}x{ray.py.iloc[0]}'
        rerun.log(path, rerun.LineStrips3D(
            segments, radii=0.001), timeless=True)


def program(path, min_x, min_y, max_x, max_y):
    print(f'Reading "{path}"...')
    rays = read_rays(path)
    print(f'Read {len(rays)} rays.')

    rays = rays[(rays.px >= min_x) & (rays.px <= max_x)
                & (rays.py >= min_y) & (rays.py <= max_y)]

    print(f'Filtered out {len(rays)} rays.')
    visualize(rays)


def main():
    parser = argparse.ArgumentParser(
        description="Visualize rays with rerun.",
        formatter_class=argparse.ArgumentDefaultsHelpFormatter)
    parser.add_argument("path", help="raylog.bin file path")
    parser.add_argument("--min", default="0x0")
    parser.add_argument("--max", default="10x10")
    args = parser.parse_args()
    [min_x, min_y] = [int(s) for s in args.min.split('x')]
    [max_x, max_y] = [int(s) for s in args.max.split('x')]
    sys.exit(program(args.path, min_x, min_y, max_x, max_y))


main()
