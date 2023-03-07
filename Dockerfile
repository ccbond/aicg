FROM rust:1.67.1
COPY . /app
WORKDIR /app
RUN cargo build --release --workspace
EXPOSE 80
CMD ["./server/target/release/aigc-server"]
