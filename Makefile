
run-basics:
	cd ./week2/1-basics/proj && cargo run

build-basics:
	cd ./week2/1-basics/proj && cargo build

run-control-flow:
	cd ./week2/2-control-flow/proj && cargo run

build-control-flow:
	cd ./week2/2-control-flow/proj && cargo build

build:
	@cargo build

run:
	@cargo run

check:
	@cargo check

doc:
	@cargo doc

test:
	@cargo test
	@cargo test --all-features

format:
	@rustup component add rustfmt 2> /dev/null
	@cargo fmt --all

format-check:
	@rustup component add rustfmt 2> /dev/null
	@cargo fmt --all -- --check

lint:
	@rustup component add clippy 2> /dev/null
	@cargo clippy
