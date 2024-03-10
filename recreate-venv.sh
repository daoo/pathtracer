#!/usr/bin/env bash

rm -rf ./venv/

python -m venv venv
./venv/bin/python -m pip install --upgrade pip setuptools wheel

./venv/bin/pip install numpy pandas rerun-sdk
