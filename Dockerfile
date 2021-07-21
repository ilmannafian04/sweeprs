FROM ekidd/rust-musl-builder:stable as builder

WORKDIR /app
COPY . .
RUN [ "cargo", "build", "--release", "-p", "cli" ]

FROM alpine:3.12

COPY --from=builder \
     /app/target/x86_64-unknown-linux-musl/release/sweeprs \
     /bin/sweeprs
ENTRYPOINT [ "sweeprs" ]
