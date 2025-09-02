FROM rust:1.86-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -m -u 1000 -s /bin/bash testuser

# 依存関係のキャッシュ用に最初にCargo.tomlとCargo.lockのみをコピー
COPY --chown=testuser:testuser application/Cargo.toml /app/Cargo.toml
COPY --chown=testuser:testuser application/Cargo.lock /app/Cargo.lock

# 各クレートのCargo.tomlをコピー
COPY --chown=testuser:testuser application/schema/Cargo.toml /app/schema/Cargo.toml
COPY --chown=testuser:testuser application/api/Cargo.toml /app/api/Cargo.toml
COPY --chown=testuser:testuser application/api_test/Cargo.toml /app/api_test/Cargo.toml

# ダミーのsrcディレクトリを作成して依存関係のビルドキャッシュを作成
RUN mkdir -p /app/schema/src /app/api/src /app/api_test/src
RUN echo "fn main() {}" > /app/schema/src/main.rs
RUN echo "fn main() {}" > /app/api/src/main.rs  
RUN echo "fn main() {}" > /app/api_test/src/main.rs

# testuserに/appディレクトリとその中身への適切な権限を付与
RUN chown -R testuser:testuser /app

USER testuser

# 依存関係のみをビルド（この段階ではソースコードは変更されないためキャッシュされる）
RUN cargo build --tests

# 実際のソースコードをコピー（この時点でソースコードの変更が反映される）
COPY --chown=testuser:testuser application/schema /app/schema
COPY --chown=testuser:testuser application/api /app/api
COPY --chown=testuser:testuser application/api_test /app/api_test

CMD ["cargo", "test", "--workspace"]