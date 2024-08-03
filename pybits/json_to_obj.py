#!/usr/bin/env python3

import argparse
import json
import numpy as np 
import sys


def read(path):
    with open(path) as f:
        return np.array(json.load(f)["triangles"])


def program(args):
    triangles = read(args.path)
    vertices = triangles.reshape(len(triangles) * 3, 3)
    print('usemtl todo')
    for vertex in vertices:
        print(f'v {vertex[0]} {vertex[1]} {vertex[2]}')
    for index in range(1, len(vertices), 3):
        print(f'f {index}// {index + 1}// {index + 2}//')


def main():
    parser = argparse.ArgumentParser(
        description="Convert JSON to OBJ",
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
    )
    parser.add_argument("path")
    args = parser.parse_args()
    sys.exit(program(args))


main()
