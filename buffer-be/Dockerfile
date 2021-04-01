FROM rust:1-alpine as builder

WORKDIR /home
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
RUN ["apk", "add", "--no-cache", "musl-dev", "postgresql-dev"]
RUN ["cargo", "build", "--release"]

FROM alpine:3

COPY --from=builder /home/target/release/buffer-be /usr/local/bin

ENTRYPOINT [ "buffer-be" ]
