FROM ekidd/rust-musl-builder:stable AS builder

RUN apt install -y clang libavcodec-dev libavformat-dev libavutil-dev pkg-config libavdevice-dev libavfilter-dev
ADD --chown=rust:rust . .
RUN ["cargo", "build", "--release"]

FROM alpine:3

RUN apk --no-cache add ca-certificates
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/buffer-tc /usr/local/bin/
