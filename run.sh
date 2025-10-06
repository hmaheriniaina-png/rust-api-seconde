#!/bin/bash

echo "Building Rust REST API..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "Build successful! Starting server..."
    ./target/release/rust-rest-api
else
    echo "Build failed. Please check the errors above."
    exit 1
fi
