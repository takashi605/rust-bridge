FROM rust:1.83-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -m -u 1000 -s /bin/bash apiuser

COPY --chown=apiuser:apiuser application/api /app/api
COPY --chown=apiuser:apiuser application/schema /app/schema

WORKDIR /app/api

USER apiuser

RUN cargo build --release

EXPOSE 8080

CMD ["cargo", "run", "--release"]