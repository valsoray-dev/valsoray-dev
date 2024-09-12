FROM rust:alpine AS builder
WORKDIR /app

RUN apk add --no-cache build-base

COPY Cargo.toml Cargo.lock ./
COPY dummy.rs ./

# Build only dependencies
RUN sed -i 's|src/main.rs|dummy.rs|' Cargo.toml
RUN cargo build --release
RUN sed -i 's|dummy.rs|src/main.rs|' Cargo.toml

COPY . .
RUN cargo build --release


FROM alpine:latest AS runner
WORKDIR /app

COPY --from=builder /app/target/release/valsoray-dev ./
COPY --from=builder /app/assets ./assets

CMD [ "./valsoray-dev" ]
