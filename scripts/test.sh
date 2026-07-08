#!/bin/bash

set -e

echo "Running HadronRNG-Pass tests..."

cargo fmt --check

cargo clippy --all-targets --all-features

cargo test

echo "All tests passed!"