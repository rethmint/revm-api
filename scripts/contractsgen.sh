#!/bin/bash

# Define input and output directories
CONTRACTS_DIR="contracts"
BUILD_DIR="build"

# Create the build directory if it doesn't exist
mkdir -p "$BUILD_DIR"

# Loop through all .sol files in the contracts directory
for contract in "$CONTRACTS_DIR"/*.sol; do
    # Get the base name of the contract file (without path and extension)
    contract_name=$(basename "$contract" .sol)
    
    # Compile the contract to generate .bin and .abi files
    solc --bin --abi -o "$BUILD_DIR" "$contract"
    
    echo "Compiled $contract_name to $BUILD_DIR"
done
