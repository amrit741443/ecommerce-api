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

RUN cargo build --release -p auth-service


# ============================================
# Stage 3: Runtime
# ============================================
FROM ubuntu:24.04 AS runtime

RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/auth-service /usr/local/bin/auth-service
EXPOSE 8001

CMD ["auth-service"]