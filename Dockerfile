FROM rust:1.86

COPY . . 

RUN cargo build --release

CMD ["./target/release/benbot-rs"]
