#!/bin/bash

cd ..
# I dont like this name, should fix eventually
OUTPUT_DIR="docs/rust"
cargo doc --target-dir "$OUTPUT_DIR" --open
echo "Finished generating code documentation"
