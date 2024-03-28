#!/usr/bin/env python3

import numpy as np
import pandas as pd
import rerun
import sys

dt = np.dtype([
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
path = sys.argv[1]
print(f'Reading "{path}"...')
rays = pd.DataFrame(np.fromfile(path, dt))
print(f'Read {len(rays)} rays.')

pixels = np.array([[100, 150], [350, 450]])

rays = rays[(rays.px >= pixels[0][0]) & (rays.px <= pixels[0][1])
            & (rays.py >= pixels[1][0]) & (rays.py <= pixels[1][1])]

print(f'Filtered out {len(rays)} rays.')

rerun.init('raytracing')
rerun.connect()
rerun.log('rays', rerun.ViewCoordinates.RIGHT_HAND_Y_UP, timeless=True)

for (_, ray) in rays.groupby(['i', 'px', 'py']):
    segments = ray[['ax', 'ay', 'az', 'bx', 'by', 'bz']].to_numpy()
    segments = segments.reshape((len(ray), 2, 3))
    path = f'rays/iter{ray.i.iloc[0]}/{ray.px.iloc[0]}x{ray.py.iloc[0]}'
    print(path)
    rerun.log(path, rerun.LineStrips3D(segments, radii=0.001), timeless=True)
