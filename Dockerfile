FROM rust:1.68-alpine as builder

RUN apk add openssl-dev musl-dev

COPY . /app
WORKDIR /app

RUN cargo build --release