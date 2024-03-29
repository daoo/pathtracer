#!/usr/bin/env python3

import argparse
import rerun
import sys
import visualize_kdtree
import visualize_rays
import visualize_triangles


def program(args):
    rerun.init('pathtracer')
    rerun.connect()
    rerun.log('world', rerun.ViewCoordinates.RIGHT_HAND_Y_UP, timeless=True)

    if args.triangles:
        print(f'Reading "{args.triangles}"...')
        triangles = visualize_triangles.read(args.triangles)
        print(f'Read {len(triangles)} triangles.')
        visualize_triangles.visualize(triangles)

    if args.raylog:
        [min_str, max_str] = args.ray_window.split('+')
        [min_x, min_y] = [int(s) for s in min_str.split('x')]
        [max_x, max_y] = [int(s) for s in max_str.split('x')]
        print(f'Reading "{args.raylog}"...')
        rays = visualize_rays.read(args.raylog)
        print(f'Read {len(rays)} rays.')
        rays = rays[(rays.px >= min_x) & (rays.px <= max_x)
                    & (rays.py >= min_y) & (rays.py <= max_y)]
        print(f'Filtered out {len(rays)} rays.')
        visualize_rays.visualize(rays, args.color_segment)

    if args.kdtree:
        print(f'Reading "{args.kdtree}"...')
        kdtree = visualize_kdtree.read(args.kdtree)
        node_count = visualize_kdtree.node_count(kdtree['root'])
        print(f'Read kdtree with {node_count} nodes.')
        visualize_kdtree.visualize(kdtree)


def main():
    parser = argparse.ArgumentParser(
        description='Visualize pathtracer data with rerun.',
        formatter_class=argparse.ArgumentDefaultsHelpFormatter)
    parser.add_argument('-t', '--triangles', help='kdtree.json file path')
    parser.add_argument('-r', '--raylog', help='raylog.bin file path')
    parser.add_argument('--color-segment', action='store_true', help='color rays by segment number instead of path number')
    parser.add_argument('-k', '--kdtree', help='kdtree.json file path')
    parser.add_argument('-w', '--ray-window', default='0x0+10x10')
    args = parser.parse_args()
    sys.exit(program(args))


main()
