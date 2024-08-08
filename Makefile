.PHONY = run run-sim build clean format lint test

run:
	cargo run

run-sim:
	cargo run --package conductor-sim

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
