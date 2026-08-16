FROM rust:1.97-slim AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./

# Cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY src ./src
COPY migrations ./migrations

RUN touch src/main.rs && cargo build --release

FROM node:24-alpine AS frontend-builder

WORKDIR /app

COPY shcut-frontend-nuxt/package*.json ./
RUN npm ci

COPY shcut-frontend-nuxt/ ./
RUN npx nuxt generate

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/shcut-rust /app/shcut
COPY --from=frontend-builder /app/.output/public /app/static
COPY --from=builder /app/migrations /app/migrations

RUN mkdir -p /app/data && chmod 777 /app/data

EXPOSE 5231

CMD ["/app/shcut"]
