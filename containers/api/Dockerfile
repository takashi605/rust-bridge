FROM rust:1.86-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -m -u 1000 -s /bin/bash apiuser

USER apiuser

RUN cargo install bacon

COPY --chown=apiuser:apiuser application/api /app/api
COPY --chown=apiuser:apiuser application/schema /app/schema

WORKDIR /app/api

EXPOSE 8080

CMD ["bacon", "run", "--headless"]
