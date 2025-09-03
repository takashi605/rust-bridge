FROM rust:1.86-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -m -u 1000 -s /bin/bash apiuser && \
    chown -R apiuser:apiuser /app

USER apiuser

RUN cargo install bacon

# 依存関係のキャッシュのためCargo.tomlとCargo.lockを先にコピー
COPY --chown=apiuser:apiuser application/Cargo.toml application/Cargo.lock ./
COPY --chown=apiuser:apiuser application/api_schema/Cargo.toml ./api_schema/
COPY --chown=apiuser:apiuser application/api/Cargo.toml ./api/
COPY --chown=apiuser:apiuser application/api_test/Cargo.toml ./api_test/
COPY --chown=apiuser:apiuser application/repositories/Cargo.toml ./repositories/

# ダミーのsrc/lib.rsを作成して依存関係をビルド
RUN mkdir -p api_schema/src api/src api_test/src repositories/src && \
    echo "fn main() {}" > api_schema/src/lib.rs && \
    echo "fn main() {}" > api/src/main.rs && \
    echo "fn main() {}" > api_test/src/main.rs && \
    echo "fn main() {}" > repositories/src/lib.rs && \
    cargo build --workspace && \
    rm -rf api_schema/src api/src api_test/src repositories/src

# 必要なソースコードのみをコピー（apiクレートに必要な部分のみ）
COPY --chown=apiuser:apiuser application/api_schema/ ./api_schema/
COPY --chown=apiuser:apiuser application/api/ ./api/
COPY --chown=apiuser:apiuser application/repositories/ ./repositories/

EXPOSE 8080

CMD ["bacon", "run", "--headless"]
