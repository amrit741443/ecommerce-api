# ============================================
# Stage 1: Planner
# ============================================
FROM lukemathwalker/cargo-chef:latest-rust-1 AS planner

WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ============================================
# Stage 2: Builder
# ============================================
FROM lukemathwalker/cargo-chef:latest-rust-1 AS builder

WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN cargo build --release -p payment-service


# ============================================
# Stage 3: Runtime
# ============================================
FROM debian:bookworm-slim AS runtime

WORKDIR /app

COPY --from=builder /app/target/release/payment-service /usr/local/bin/payment-service
EXPOSE 8005

CMD ["payment-service"]