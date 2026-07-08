#!/bin/bash

set -e

echo "Running fuzz tests..."

cargo fuzz run generator
cargo fuzz run password
cargo fuzz run validator

echo "Fuzz testing completed!"