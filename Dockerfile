FROM rust:1.85 AS builder

WORKDIR /usr/src/myapp

RUN apt-get update && apt-get install -y libpq-dev
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libpq5 \
    openssl \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/axum_template /usr/local/bin/myapp

EXPOSE 3000

CMD ["myapp"]
