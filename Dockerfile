# 1 : Build stage
FROM rust:1.93-slim-bookworm as builder
ARG BIN_NAME=3_current
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
# Create the directory structure matching your project
RUN mkdir -p src/bin/${BIN_NAME} && echo "fn main() {}" > src/bin/${BIN_NAME}/main.rs

RUN cargo build --release --bin ${BIN_NAME}

COPY . .
# Invalidate the cache for the correct file path
RUN touch src/bin/${BIN_NAME}/main.rs
RUN cargo build --release --bin ${BIN_NAME}
ENV SQLX_OFFLINE=true
# 2 : Runtime stage
FROM debian:bookworm-slim
ARG BIN_NAME=3_current
RUN apt-get update && apt-get install -y ca-certificates curl && rm -rf /var/lib/apt/lists/*
WORKDIR /app

COPY --from=builder /app/target/release/${BIN_NAME} /app/rust_api

EXPOSE 6669
CMD ["./rust_api"]
