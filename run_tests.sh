#!/bin/bash

# Set the path to your test files directory
TEST_DIR="test"

# Iterate over each JSON file in the tests directory
for file in "$TEST_DIR"/*.json; do
    echo "Testing file: $file"
    cargo run -- "$file"

    # Capture the exit code of the last command
    if [ $? -eq 0 ]; then
        echo "Result: Valid JSON"
    else
        echo "Result: Invalid JSON"
    fi
    echo
done
