FROM rust:1.86 as build

RUN USER=root cargo new --bin benbot-rs
WORKDIR /benbot-rs

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/benbot_rs*
RUN cargo build --release

FROM rust:1.86

COPY --from=build /benbot-rs/target/release/benbot-rs .

CMD ["./benbot-rs"]
