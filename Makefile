SHELL := /bin/bash

bot: Cargo.toml Cargo.lock src/main.rs
	cargo build
	source env.sh
	cargo run
