FROM rust:1.67.1

WORKDIR /app
COPY . .
RUN cargo build --release --workspace
EXPOSE 80
# CMD ["./server/target/release/aigc-server"]
