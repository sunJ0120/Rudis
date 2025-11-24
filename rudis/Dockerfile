FROM rust:latest

RUN apt-get update && apt-get install -y redis-tools && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY . .

RUN cargo build --release

EXPOSE 6380

CMD ["./target/release/rudis"]