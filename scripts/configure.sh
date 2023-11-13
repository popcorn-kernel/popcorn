#!/usr/bin/env bash

# configures the project for build
rustup install nightly
cargo install bootimage
rustup component add llvm-tools-preview
rustup component add rust-src
