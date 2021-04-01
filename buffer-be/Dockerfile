FROM ekidd/rust-musl-builder:stable AS builder

ADD --chown=rust:rust . .
RUN ["cargo", "build", "--release"]

FROM alpine:3

RUN apk --no-cache add ca-certificates
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/buffer-be /usr/local/bin/

ENTRYPOINT ["buffer-be"]
