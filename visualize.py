#!/usr/bin/env python3

import pandas as pd
import rerun

rays = pd.read_csv('raylog.txt', delimiter=',', skiprows=2, names=['px', 'py', 'ax', 'ay', 'az', 'bx', 'by', 'bz'])

rerun.init('ray_visualization')
rerun.connect()

# segments = rays[['ax', 'ay', 'az', 'bx', 'by', 'bz']].to_numpy()
# segments = segments.reshape((len(rays), 2, 3))
# labels = rays[['px', 'py']].astype(str).agg('x'.join, axis=1)

for (_, ray) in rays.groupby(['px', 'py']):
    segments = ray[['ax', 'ay', 'az', 'bx', 'by', 'bz']].to_numpy()
    segments = segments.reshape((len(ray), 2, 3))
    rerun.log(
            f'rays/{int(ray.px.iloc[0])}x{int(ray.py.iloc[0])}',
            rerun.LineStrips3D(segments, radii=0.001),
            timeless=True)
