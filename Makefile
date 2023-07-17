build:
	cargo bootimage --target arch/x86_64-arch.json

install:
	cargo install bootimage
	rustup component add llvm-tools-preview

run: build
	cargo run

clean:
	cargo clean

debug: build
	bash -c "qemu-system-x86_64 -s -S -drive format=raw,file=target/x86_64-arch/debug/bootimage-popcorn.bin &"

rebuild: clean build

test:
	cargo test --test heap_allocation
	cargo test --test stack_overflow

.PHONY: build