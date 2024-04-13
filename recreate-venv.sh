#!/usr/bin/env bash

rm -rf ./venv/

uv venv venv

source ./venv/bin/activate
uv pip install numpy pandas rerun-sdk ruff "python-lsp-server[rope]"
