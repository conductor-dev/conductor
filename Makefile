.PHONY = run build clean doc format lint test test-doc

run:
	cargo run

build:
	cargo build

clean:
	rm -rf target

format:
	cargo fmt

lint:
	cargo clippy -- -D warnings

test:
	cargo test --all
