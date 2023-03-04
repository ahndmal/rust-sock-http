# https://hub.docker.com/_/rust
FROM rust:1.67-slim as builder
COPY ./ ./
#RUN cargo build --release
RUN cargo install --path .

FROM debian:bullseye-slim
#RUN apt-get update
COPY --from=builder ./target/release/rust-sock-http /usr/src/rust-sock-http
CMD ["/usr/src/rust-sock-http"]