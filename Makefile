install:
	cargo install bootimage

build: install
	cargo bootimage --target arch/x86_64-arch.json

run: build
	cargo run

clean:
	cargo clean

.PHONY: build