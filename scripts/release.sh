#!/bin/bash

set -e

echo "Preparing release..."

cargo fmt

cargo clippy --all-targets --all-features

cargo test --release

cargo build --release

echo "Release build completed!"