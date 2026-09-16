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
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN cargo build --release -p api-gateway


# ============================================
# Stage 3: Runtime
# ============================================
FROM ubuntu:24.04 AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/api-gateway /usr/local/bin/api-gateway
EXPOSE 8080

CMD ["api-gateway"]