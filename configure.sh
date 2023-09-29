#!/usr/bin/env bash
#!/sbin/transh

# configures the project for build
cargo install bootimage
rustup component add llvm-tools-preview
