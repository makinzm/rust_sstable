# Reference: [rust - Official Image | Docker Hub](https://hub.docker.com/_/rust)
FROM rust:1.67 as builder
WORKDIR /usr/src/rust_sstable
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
# TODO: The following code fails though the official document says it should work.
#  ERROR: failed to solve: process "/bin/sh -c apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*" did not complete successfully: exit code: 100
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/rust_sstable/target/debug/rust_sstable /usr/local/bin/rust_sstable
CMD ["rust_sstable"]
