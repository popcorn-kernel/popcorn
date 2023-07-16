install:
	cargo install bootimage

build:
	cargo bootimage --target arch/x86_64-arch.json

run: build
	cargo run

clean:
	cargo clean

debug: build
	bash -c "qemu-system-x86_64 -s -S -drive format=raw,file=target/x86_64-arch/debug/bootimage-popcorn.bin &"

make test: build
	cargo test

.PHONY: build