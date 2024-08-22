FROM lukemathwalker/cargo-chef:latest-rust-1.80.0 AS chef 
WORKDIR /app 
RUN apt update && apt install lld clang -y 
FROM chef AS planner 
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
# Builder Stage
# FROM rust:1.80.0 AS builder
# WORKDIR /app 
# RUN apt update && apt install lld clang -y 
# COPY . . 
# sqlx is offline by default
ENV SQLX_OFFLINE=true
ENV LLM_LS_TARGET=x86_64-unknown-linux-musl
RUN cargo build --release --bin enl

# Runtime Stage
FROM rust:1.80.0-slim AS runtime
# FROM debian:bullseye-slim AS runtime
WORKDIR /app 
# RUN  apt-get update -y \
# 	&& apt-get install -y --no-install-recommends openssl ca-certificates \
# 	&& apt-get install libc6 -y \
# 	&& apt-get --fix-broken install -y \
# 	&& apt-get autoremove -y \
# 	&& apt-get clean -y \
# 	&& rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/enl enl
COPY configuration configuration
ENV APP_ENVIRONMENT=production
ENTRYPOINT ["./enl"]

