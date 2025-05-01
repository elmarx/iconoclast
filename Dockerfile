FROM rust:1.86.0 AS builder
ARG REVISION
WORKDIR /usr/src

RUN apt-get update && apt-get install -y libssl-dev libsasl2-dev clang

RUN mkdir -p \
    repository/src  \
    uservice

COPY Cargo.toml Cargo.lock ./
COPY repository/Cargo.toml repository/
COPY uservice/Cargo.toml uservice/

# compile all dependencies with a dummy for improved caching
RUN mkdir -p uservice/src/bin && \
  touch repository/src/lib.rs && \
  echo "fn main() { println!(\"Dummy\"); }" > uservice/src/bin/dummy.rs && \
  cargo build --release && \
  rm -rf uservice/src

# now compile the real code
COPY ./config.default.toml .
COPY repository repository
COPY uservice uservice

# update the timestamps so cargo picks up the actual code
RUN touch repository/src/lib.rs

RUN cargo install --locked --path uservice --root /usr/local

FROM debian:stable-slim

RUN apt-get update && apt-get install -y libssl3 libsasl2-2
ENV ICONOCLAST_LOGGING=json
RUN adduser --system iconoclast
COPY --from=builder /usr/local/bin/iconoclast /usr/local/bin

USER iconoclast
WORKDIR /home/iconoclast

CMD [ "iconoclast" ]
