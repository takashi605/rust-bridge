FROM rust:1.86-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

RUN useradd -m -u 1000 -s /bin/bash apiuser && \
    chown -R apiuser:apiuser /app

USER apiuser

# baconを軽量化：cargo-watchの代替として最小限のインストール
RUN cargo install bacon --no-default-features

# 依存関係のキャッシュのためCargo.toml（とCargo.lockがあれば）を先にコピー
COPY --chown=apiuser:apiuser application/Cargo.* ./
COPY --chown=apiuser:apiuser application/api_schema/Cargo.toml ./api_schema/
COPY --chown=apiuser:apiuser application/api/Cargo.toml ./api/
COPY --chown=apiuser:apiuser application/api_test/Cargo.toml ./api_test/
COPY --chown=apiuser:apiuser application/repositories/Cargo.toml ./repositories/
COPY --chown=apiuser:apiuser application/mailer/Cargo.toml ./mailer/

# ダミーのsrc/lib.rsを作成して依存関係をビルド（デバッグ用）
RUN mkdir -p api_schema/src api/src api_test/src repositories/src mailer/src && \
    echo "pub fn dummy() {}" > api_schema/src/lib.rs && \
    echo "fn main() {}" > api/src/main.rs && \
    echo "fn main() {}" > api_test/src/main.rs && \
    echo "pub fn dummy() {}" > repositories/src/lib.rs && \
    echo "pub fn dummy() {}" > mailer/src/lib.rs && \
    cargo build --workspace && \
    rm -rf api_schema/src api/src api_test/src repositories/src mailer/src

# 必要なソースコードのみをコピー（ソースコードディレクトリのみ）
COPY --chown=apiuser:apiuser application/api_schema/src/ ./api_schema/src/
COPY --chown=apiuser:apiuser application/api/src/ ./api/src/
COPY --chown=apiuser:apiuser application/repositories/src/ ./repositories/src/
COPY --chown=apiuser:apiuser application/mailer/src/ ./mailer/src/

EXPOSE 8080

CMD ["bacon", "run", "--headless"]
