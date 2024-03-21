#!/usr/bin/env python3

import numpy as np
import pandas as pd
import rerun

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
rays = pd.DataFrame(np.fromfile('/tmp/raylog0.bin', dt))
print(f'Read {len(rays)} rays...')
print(rays)

rays = rays[(rays.px >= 670) & (rays.px <= 675) & (rays.py >= 814) & (rays.py <= 820)]
print(rays)

rerun.init('raytracing')
rerun.connect()
rerun.log('rays', rerun.ViewCoordinates.RIGHT_HAND_Y_UP, timeless=True)

for (f, ray) in rays.groupby(['i', 'px', 'py']):
    segments = ray[['ax', 'ay', 'az', 'bx', 'by', 'bz']].to_numpy()
    segments = segments.reshape((len(ray), 2, 3))
    path = f'rays/iter{int(ray.i.iloc[0])}/{int(ray.px.iloc[0])}x{int(ray.py.iloc[0])}'
    rerun.log(path, rerun.LineStrips3D(segments, radii=0.001), timeless=True)
