#!/usr/bin/env python3

import argparse
import rerun
import sys
import visualize_fails
import visualize_kdtree
import visualize_rays
import visualize_triangles


def parse_rectangle(s):
    try:
        [min_str, max_str] = s.split("+")
        [x1, y1] = [int(s) for s in min_str.split("x")]
        [x2, y2] = [int(s) for s in max_str.split("x")]
        return [x1, y1, x2, y2]
    except ValueError:
        return None


def program(args):
    rerun.init("pathtracer")
    rerun.connect()
    rerun.log("world", rerun.ViewCoordinates.RIGHT_HAND_Y_UP, timeless=True)

    if args.triangles:
        print(f'Reading "{args.triangles}"...')
        triangles = visualize_triangles.read(args.triangles)
        print(f"Read {len(triangles)} triangles.")
        visualize_triangles.visualize(triangles)

    if args.ray_log:
        print(f'Reading "{args.ray_log}"...')
        rays = visualize_rays.read(args.ray_log)
        print(f"Read {len(rays)} rays.")
        ray_window = parse_rectangle(args.ray_window) if args.ray_window else None
        if ray_window:
            [x1, y1, x2, y2] = ray_window
            rays = rays[
                (rays.px >= x1) & (rays.px <= x2) & (rays.py >= y1) & (rays.py <= y2)
            ]
            print(
                f"Filtered out {len(rays)} rays from {[x1, y1]} to {[x2, y2]} (inclusive)."
            )
        if args.ray_mode == "single":
            visualize_rays.visualize_as_one_entity(rays)
        elif args.ray_mode == "path":
            visualize_rays.visualize_grouped_per_path(rays)

    if args.ray_fails:
        print(f'Reading "{args.ray_fails}"...')
        rays = visualize_fails.read(args.ray_fails)
        print(f"Read {len(rays)} rays.")
        visualize_fails.visualize(rays)

    if args.kdtree:
        print(f'Reading "{args.kdtree}"...')
        kdtree = visualize_kdtree.read(args.kdtree)
        node_count = visualize_kdtree.node_count(kdtree["root"])
        print(f"Read kdtree with {node_count} nodes.")
        visualize_kdtree.visualize(kdtree)


def main():
    parser = argparse.ArgumentParser(
        description="Visualize pathtracer data with rerun.",
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
    )
    parser.add_argument("-t", "--triangles", help="kdtree.json file path")
    parser.add_argument("-f", "--ray-fails", help="rayfails.bin file path")
    parser.add_argument("-k", "--kdtree", help="kdtree.json file path")

    parser.add_argument("-r", "--ray-log", help="raylog.bin file path")
    parser.add_argument(
        "-m",
        "--ray-mode",
        default="single",
        help="entity grouping mode [path | single]",
    )
    parser.add_argument("-w", "--ray-window")
    args = parser.parse_args()
    sys.exit(program(args))


main()
