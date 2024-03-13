#!/usr/bin/env python3

import pandas as pd
import rerun

rays = pd.read_csv('raylog.txt', delimiter=',', skiprows=2, names=['i', 'px', 'py', 'ax', 'ay', 'az', 'bx', 'by', 'bz'])

rerun.init('ray_visualization')
rerun.connect()

rerun.log('rays', rerun.ViewCoordinates.RIGHT_HAND_Y_UP, timeless=True)

for (_, ray) in rays.groupby(['i', 'px', 'py']):
    segments = ray[['ax', 'ay', 'az', 'bx', 'by', 'bz']].to_numpy()
    segments = segments.reshape((len(ray), 2, 3))
    path = f'rays/iter{int(ray.i.iloc[0])}/{int(ray.px.iloc[0])}x{int(ray.py.iloc[0])}'
    rerun.log(path, rerun.LineStrips3D(segments, radii=0.001), timeless=True)
