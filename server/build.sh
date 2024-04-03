#!/bin/bash

cd ../ui/
./build.sh
cd ../server/
RUST_LOG="trace" cargo run --release
