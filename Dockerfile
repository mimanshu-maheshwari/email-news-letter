# Builder Stage
FROM rust:1.77.0 AS builder
WORKDIR /app 
RUN apt update && apt install lld clang -y 
COPY . . 
# sqlx is offline by default
ENV SQLX_OFFLINE=true
RUN cargo build --release 

# Runtime Stage
#FROM rust:1.77.0-slim AS runtime
FROM debian:bullseye-slim AS runtime
WORKDIR /app 
RUN apt-get update -y \
	&& apt-get install -y --no-install-recommends openssl ca-certificates \
	&& apt-get autoremove -y \
	&& apt-get clean -y \
	&& rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/enl enl
COPY configuration configuration
ENV APP_ENVIRONMENT=production
ENTRYPOINT ["./enl"]

