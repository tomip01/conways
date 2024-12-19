build:
	cargo build
run:
	cargo run
test:
	cargo test
check:
	cargo check
lint:
	cargo clippy -- -D warnings

.PHONY: build run test check lint
