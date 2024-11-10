.PHONY = run build clean format lint test

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
