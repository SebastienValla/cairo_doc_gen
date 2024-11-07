#!/bin/bash

# Clone the cairo_doc_gen repository if it doesn't already exist
if [ ! -d "cairo_doc_gen" ]; then
    echo "Cloning cairo_doc_gen..."
    git clone https://github.com/SebastienValla/cairo_doc_gen.git
fi

# Navigate to cairo_doc_gen and build the docgen binary
cd cairo_doc_gen
cargo build --bin docgen

# Run the docgen binary
cargo run --bin docgen
