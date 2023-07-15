install:
	cargo install bootimage
	rustup component add llvm-tools-preview

build:
	cargo bootimage --target arch/x86_64-arch.json

run: build
	cargo run

clean:
	cargo clean

.PHONY: build
