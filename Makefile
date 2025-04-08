include .env

bot: Cargo.toml Cargo.lock src/main.rs
	cargo build
	BENBOT_TOKEN=${BENBOT_TOKEN} cargo run
