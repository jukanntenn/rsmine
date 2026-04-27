FROM rust:1-bookworm

RUN cargo install cargo-watch

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY backend/Cargo.toml backend/Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build
RUN rm -rf src

RUN mkdir -p /app/data/files

EXPOSE 3001

CMD ["cargo", "watch", "-x", "run"]
