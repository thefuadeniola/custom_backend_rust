FROM ubuntu:22.04
FROM messense/rust-musl-cross:x86_64-musl as builder

# Install OpenSSL development headers
RUN apt-get update && apt-get install -y libssl-dev pkg-config

ENV SQLX_OFFLINE=true

WORKDIR /blog-server

# Copy the source code
COPY . .


RUN cargo build --release --target x86_64-unknown-linux-musl

# Create a new minimal image
FROM scratch

COPY --from=builder /blog-server/target/x86_64-unknown-linux-musl/release/blog-server /blog-server
ENTRYPOINT ["/blog-server"]
EXPOSE 8000

