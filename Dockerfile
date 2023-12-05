FROM rust:alpine as builder
WORKDIR /app

RUN apk add build-base

COPY Cargo.toml ./
COPY Cargo.lock ./
COPY dummy.rs ./

RUN sed -i 's|src/main.rs|dummy.rs|' Cargo.toml

RUN cargo build --release

RUN sed -i 's|dummy.rs|src/main.rs|' Cargo.toml

COPY . .

RUN cargo build --release

RUN mkdir dist
RUN mv ./target/release/valsoray-dev ./dist/
RUN mv ./assets ./dist/


FROM alpine:latest as runner
WORKDIR /app

COPY --from=builder /app/dist ./

CMD [ "./valsoray-dev" ]