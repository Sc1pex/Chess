#!/bin/sh

wasm-pack build
cd pkg
bun link
cd ../../ui
bun install
