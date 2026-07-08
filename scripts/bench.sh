#!/bin/bash

set -e

echo "Running benchmarks..."

cargo bench

echo "Benchmark completed!"