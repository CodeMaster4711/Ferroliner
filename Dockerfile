# Stage 1: Build frontend
FROM node:22-alpine AS frontend-builder

WORKDIR /frontend

COPY frontend/package.json frontend/package-lock.json* ./
RUN npm install

COPY frontend/ ./

ARG JWT_SECRET=change-me-in-production
ARG BACKEND_URL=http://localhost:8000
ENV JWT_SECRET=$JWT_SECRET
ENV BACKEND_URL=$BACKEND_URL

RUN npm run build

# Stage 2: Build backend
FROM rust:1.88-slim AS backend-builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY backend/Cargo.toml backend/Cargo.lock* ./
COPY backend/api/Cargo.toml api/
COPY backend/entity/Cargo.toml entity/
COPY backend/migration/Cargo.toml migration/

RUN mkdir -p api/src entity/src migration/src && \
    echo 'fn main() {}' > api/src/main.rs && \
    echo '' > entity/src/lib.rs && \
    printf 'pub use sea_orm_migration::prelude::*;\npub struct Migrator;\n#[async_trait::async_trait]\nimpl sea_orm_migration::MigratorTrait for Migrator {\n    fn migrations() -> Vec<Box<dyn sea_orm_migration::MigrationTrait>> { vec![] }\n}' > migration/src/lib.rs && \
    printf '#[tokio::main]\nasync fn main() { sea_orm_migration::cli::run_cli(migration::Migrator).await; }' > migration/src/main.rs

RUN cargo build --release 2>/dev/null || true

COPY backend/api/src api/src
COPY backend/entity/src entity/src
COPY backend/migration/src migration/src

RUN touch api/src/main.rs entity/src/lib.rs migration/src/lib.rs && \
    cargo build --release

# Stage 3: Runtime
FROM node:22-bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=backend-builder /app/target/release/api /app/api
COPY --from=frontend-builder /frontend/build /app/frontend/build
COPY --from=frontend-builder /frontend/package.json /app/frontend/package.json
COPY --from=frontend-builder /frontend/node_modules /app/frontend/node_modules

RUN mkdir -p /data

COPY docker-entrypoint.sh /app/docker-entrypoint.sh
RUN chmod +x /app/docker-entrypoint.sh

EXPOSE 3000
EXPOSE 8000

ENTRYPOINT ["/app/docker-entrypoint.sh"]
