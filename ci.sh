#!/usr/bin/env bash
set -euo pipefail

# Rust checks used by pre-push
cargo fmt --all -- --check
cargo build --all-targets
cargo test --all
cargo clippy --all-targets --all-features -- -D warnings
