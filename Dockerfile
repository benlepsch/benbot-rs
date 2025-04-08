FROM rust:1.86
LABEL authors="benlepsch"

COPY . .

RUN cargo build --release

CMD ["./target/release/benbot-rs"]
