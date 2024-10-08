#!/bin/bash

SCHEMA_DIR="types"

for fbs_file in "$SCHEMA_DIR"/flatbuffer/*.fbs; do
    if [ -f "$fbs_file" ]; then
        echo "Generating Go code for $fbs_file..."
        flatc --go -o types/go "$fbs_file"

        if [ $? -ne 0 ]; then
            echo "Error generating Go code for $fbs_file!"
            exit 1
        fi

        echo "Generating Rust code for $fbs_file..."
        flatc --rust -o types/rust/src/ "$fbs_file"

        if [ $? -ne 0 ]; then
            echo "Error generating Rust code for $fbs_file!"
            exit 1
        fi

        echo "Code generation completed for $fbs_file!"
    else
        echo "No .fbs files found in $SCHEMA_DIR."
        exit 1
    fi
done

echo "All code generation completed successfully!"
