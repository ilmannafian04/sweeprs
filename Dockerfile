FROM rust:1.49-alpine3.12 as builder

WORKDIR /app
COPY . .
RUN ["cargo", "build", "--release"]

FROM alpine:3.12

COPY --from=builder /app/target/release/sweeprs /bin/sweeprs
ENTRYPOINT [ "/bin/ash", "-c", "sweeprs" ]
