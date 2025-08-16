FROM rust:1.83-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY application/api_test /app/api_test

WORKDIR /app/api_test

RUN cargo build --tests

CMD ["cargo", "test"]