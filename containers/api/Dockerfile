FROM rust:1.86-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install bacon

COPY application/api /app/api
COPY application/schema /app/schema

WORKDIR /app/api

RUN cargo build --release

RUN useradd -m -u 1000 -s /bin/bash apiuser

RUN chown -R apiuser:apiuser /app
RUN chown -R apiuser:apiuser /app/schema
RUN chown -R apiuser:apiuser /usr/local/cargo

USER apiuser

EXPOSE 8080

CMD ["bacon", "run", "--headless"]