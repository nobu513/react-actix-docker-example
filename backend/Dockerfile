FROM rust:1.63 AS builder

RUN cargo new --bin app
WORKDIR /app

COPY Cargo.toml .

RUN cargo install --path .

RUN rm ./src/*.rs
COPY ./src ./src
RUN cargo build --release 

FROM rust:1.63

COPY --from=builder /app/target/release/launch_api .

CMD ["./launch_api"]


