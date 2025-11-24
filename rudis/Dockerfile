FROM rust:latest

WORKDIR /app

COPY . .

RUN cargo build --release

EXPOSE 6379

CMD ["./target/release/rudis"]