#!/usr/bin/env bash

# configures the project for build
cargo install bootimage
rustup component add llvm-tools-preview
rustup component add rust-src
