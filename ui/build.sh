#!/bin/sh

bun install
cd ../chess-lib
./build.sh
cd ../ui
bun run build
