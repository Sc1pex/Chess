#!/bin/bash

cd ../ui/
bun vite build
cd ../server/
RUST_LOG="trace" cargo run --release
