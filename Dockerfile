FROM rust:1.85

WORKDIR /app

COPY . .

RUN cargo build --release

CMD ["./target/release/blackout-engine"]