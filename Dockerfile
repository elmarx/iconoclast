FROM rust:1.86.0 AS builder
ARG REVISION
WORKDIR /usr/src

RUN apt-get update && apt-get install -y libssl-dev libsasl2-dev clang

RUN mkdir -p \
    infra/src \
    logic/src \
    model/src \
    repository/src \
    uservice

COPY Cargo.toml Cargo.lock ./
COPY infra/Cargo.toml infra/
COPY logic/Cargo.toml logic/
COPY model/Cargo.toml model/
COPY repository/Cargo.toml repository/
COPY uservice/Cargo.toml uservice/

# compile all dependencies with a dummy for improved caching
RUN mkdir -p uservice/src/bin && \
  touch infra/src/lib.rs && \
  touch logic/src/lib.rs && \
  touch model/src/lib.rs && \
  touch repository/src/lib.rs && \
  echo "fn main() { println!(\"Dummy\"); }" > uservice/src/bin/dummy.rs && \
  cargo build --release && \
  rm -rf uservice/src

# now compile the real code
COPY ./config.default.toml .
COPY infra infra
COPY logic logic
COPY model model
COPY repository repository
COPY uservice uservice

# update the timestamps so cargo picks up the actual code
RUN touch infra/src/lib.rs logic/src/lib.rs model/src/lib.rs repository/src/lib.rs

RUN cargo install --locked --path uservice --root /usr/local

FROM debian:stable-slim

RUN apt-get update && apt-get install -y libssl3 libsasl2-2
ENV ICONOCLAST_LOGGING=json
RUN adduser --system iconoclast
COPY --from=builder /usr/local/bin/iconoclast /usr/local/bin

USER iconoclast
WORKDIR /home/iconoclast

CMD [ "iconoclast" ]
