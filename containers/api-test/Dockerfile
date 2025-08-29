FROM rust:1.83-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -m -u 1000 -s /bin/bash testuser

COPY --chown=testuser:testuser application/api_test /app/api_test

WORKDIR /app/api_test

USER testuser

RUN cargo build --tests

CMD ["cargo", "test"]