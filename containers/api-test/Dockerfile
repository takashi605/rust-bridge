FROM rust:1.86-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -m -u 1000 -s /bin/bash testuser && \
    chown -R testuser:testuser /app

USER testuser

# 依存関係のキャッシュのためCargo.toml（とCargo.lockがあれば）を先にコピー
COPY --chown=testuser:testuser application/Cargo.* ./
COPY --chown=testuser:testuser application/api_schema/Cargo.toml ./api_schema/
COPY --chown=testuser:testuser application/api/Cargo.toml ./api/
COPY --chown=testuser:testuser application/api_test/Cargo.toml ./api_test/
COPY --chown=testuser:testuser application/repositories/Cargo.toml ./repositories/
COPY --chown=testuser:testuser application/mailer/Cargo.toml ./mailer/

# ダミーのsrc/lib.rsを作成して依存関係をビルド
RUN mkdir -p api_schema/src api/src api_test/src repositories/src mailer/src && \
    echo "pub fn dummy() {}" > api_schema/src/lib.rs && \
    echo "fn main() {}" > api/src/main.rs && \
    echo "fn main() {}" > api_test/src/main.rs && \
    echo "pub fn dummy() {}" > repositories/src/lib.rs && \
    echo "pub fn dummy() {}" > mailer/src/lib.rs && \
    cargo build --workspace --tests && \
    rm -rf api_schema/src api/src api_test/src repositories/src mailer/src

# 必要なソースコードのみをコピー（ソースコードディレクトリのみ）
COPY --chown=testuser:testuser application/api_schema/src/ ./api_schema/src/
COPY --chown=testuser:testuser application/api_test/src/ ./api_test/src/
COPY --chown=testuser:testuser application/repositories/src/ ./repositories/src/
COPY --chown=testuser:testuser application/mailer/src/ ./mailer/src/

CMD ["cargo", "test", "--workspace"]