install:
	cargo install bootimage

build:
	cargo bootimage --target arch/x86_64-arch.json

run: build
	cargo run

clean:
	cargo clean

.PHONY: build
