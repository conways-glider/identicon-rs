FROM rust:1.62.1 as builder

COPY . .
RUN rustup target add x86_64-unknown-linux-musl
# Sets the environment variable for the cargo build command that follows.
ENV PKG_CONFIG_ALLOW_CROSS=1
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:latest

COPY --from=builder /target/x86_64-unknown-linux-musl/release/identicon-server .

CMD ["/identicon-server"]
