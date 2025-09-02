FROM rust:1.86-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -m -u 1000 -s /bin/bash testuser

COPY --chown=testuser:testuser application/Cargo.toml /app/Cargo.toml
COPY --chown=testuser:testuser application/Cargo.lock /app/Cargo.lock
COPY --chown=testuser:testuser application/api_test /app/api_test
COPY --chown=testuser:testuser application/schema /app/schema
COPY --chown=testuser:testuser application/api /app/api

WORKDIR /app

# testuserに/appディレクトリとその中身への適切な権限を付与
RUN chown -R testuser:testuser /app

USER testuser

# targetディレクトリを事前に作成して権限を設定
RUN mkdir -p /app/target && cargo build --tests

CMD ["cargo", "test", "--workspace"]