# We use the latest Rust stable release as base image 
FROM rust:1.77.0

WORKDIR /app 

RUN apt update && apt install lld clang -y 

COPY . . 

# sqlx is offline by default
ENV SQLX_OFFLINE true

RUN cargo build --release 

ENTRYPOINT ["./target/release/enl"]

