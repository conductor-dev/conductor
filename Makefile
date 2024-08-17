.PHONY = run run-sim build clean format lint test

run:
	cargo run

build:
	cargo build

build-sim:
	cargo build --package conductor-sim

clean:
	rm -rf target

format:
	cargo fmt

lint:
	cargo clippy -- -D warnings

test:
	cargo test --all
