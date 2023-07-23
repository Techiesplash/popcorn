install:
	cargo install bootimage
	rustup component add llvm-tools-preview
build:
	cargo build --target x86_64-unknown-none

run: build
	cargo run

clean:
	cargo clean

debug: build
	bash -c "qemu-system-x86_64 -s -S -drive format=raw,file=target/x86_64-unknown-none/debug/popcorn &"

rebuild: clean build

test:
	cargo test --test syscall_test
	cargo test --test heap_allocation
	cargo test --test stack_overflow

rebuild-debug: clean debug

.PHONY: build