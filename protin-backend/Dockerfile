FROM docker.io/rust:latest as builder

WORKDIR /app

COPY . .

RUN cargo install --path .

FROM docker.io/debian:bookworm-slim

RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/protin /usr/local/bin/protin

CMD ["protin"]
