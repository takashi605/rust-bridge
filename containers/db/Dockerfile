FROM postgres:16

# ユーザーと権限の設定
RUN useradd -m -u 1000 -s /bin/bash dbuser

# 初期化スクリプト用ディレクトリの準備
RUN mkdir -p /docker-entrypoint-initdb.d
RUN chown postgres:postgres /docker-entrypoint-initdb.d

# 初期化スクリプトをコピー
COPY containers/db/init/*.sql /docker-entrypoint-initdb.d/
RUN chown postgres:postgres /docker-entrypoint-initdb.d/*.sql

# PostgreSQLの設定
ENV POSTGRES_DB=graphql_api
ENV POSTGRES_USER=postgres
ENV POSTGRES_PASSWORD=postgres

# ポート公開
EXPOSE 5432

# データディレクトリの権限設定
RUN mkdir -p /var/lib/postgresql/data
RUN chown postgres:postgres /var/lib/postgresql/data
RUN chmod 700 /var/lib/postgresql/data