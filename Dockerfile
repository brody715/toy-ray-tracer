# syntax = docker/dockerfile:1.3

FROM rust:1.59.0 AS builder

COPY ci/rust/config.toml /usr/local/cargo/config

WORKDIR /app

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    CARGO_HTTP_MULTIPLEXING=false cargo build -p toy_ray_tracer --release

FROM ubuntu:20.04

RUN sed -i -e 's/ports.ubuntu.com/mirrors.tuna.tsinghua.edu.cn/g' /etc/apt/sources.list
# RUN apt-get update && apt-get install -y libc-bin && rm -rf /var/lib/apt/lists/*

WORKDIR /app
RUN mkdir -p /app/output

ENV RUST_BACKTRACE=full
COPY --from=builder /app/assets /app/assets
COPY --from=builder /app/target/release/toy_ray_tracer /usr/local/bin/toy_ray_tracer
ENTRYPOINT ["toy_ray_tracer"]
