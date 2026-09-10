FROM  rust:latest AS builder

WORKDIR /app
COPY . .
RUN cargo build --release -p user-service

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/user-service /app/user-service

EXPOSE 4000
CMD ["/app/user-service"]
