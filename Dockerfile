FROM ekidd/rust-musl-builder:stable as builder

WORKDIR /app
COPY . .
RUN ["cargo", "build", "--release"]

FROM alpine:3.12

COPY --from=builder /app/target/release/sweeprs /bin/sweeprs
ENTRYPOINT [ "sweeprs" ]
