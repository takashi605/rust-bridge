FROM rust:1.86-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -m -u 1000 -s /bin/bash testuser

USER testuser

# 依存関係のキャッシュのためCargo.tomlとCargo.lockを先にコピー
COPY --chown=testuser:testuser application/Cargo.toml application/Cargo.lock ./
COPY --chown=testuser:testuser application/api_schema/Cargo.toml ./api_schema/
COPY --chown=testuser:testuser application/api/Cargo.toml ./api/
COPY --chown=testuser:testuser application/api_test/Cargo.toml ./api_test/

# ダミーのsrc/lib.rsを作成して依存関係をビルド
RUN mkdir -p api_schema/src api/src api_test/src && \
    echo "fn main() {}" > api_schema/src/lib.rs && \
    echo "fn main() {}" > api/src/main.rs && \
    echo "fn main() {}" > api_test/src/main.rs && \
    cargo build --workspace --tests && \
    rm -rf api_schema/src api/src api_test/src

# 実際のソースコードをコピー
COPY --chown=testuser:testuser application/ ./

CMD ["cargo", "test", "--workspace"]