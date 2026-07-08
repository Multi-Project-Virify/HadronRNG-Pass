!/bin/bash

echo "Starting HadronRNG fuzz tests..."

echo ""
echo "[1/3] Generator fuzz"
cargo fuzz run generator

echo ""
echo "[2/3] Password fuzz"
cargo fuzz run password

echo ""
echo "[3/3] Validator fuzz"
cargo fuzz run validator

echo ""
echo "Fuzz testing completed!"