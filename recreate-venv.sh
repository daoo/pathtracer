#!/usr/bin/env bash

rm -rf ./.venv/

uv venv .venv
uv pip install numpy pandas matplotlib rerun-sdk==0.18.2 ruff-lsp pyright
