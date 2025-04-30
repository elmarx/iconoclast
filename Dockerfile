FROM rust:1.86.0 AS builder
ARG REVISION
WORKDIR /usr/src

RUN apt-get update && apt-get install -y libssl-dev libsasl2-dev clang

COPY Cargo.toml Cargo.lock ./

# compile all dependencies with a dummy for improved caching
RUN mkdir -p src/bin && \
  echo "fn main() { println!(\"Dummy\"); }" > src/bin/dummy.rs && \
  cargo build --release && \
  rm -rf src

# now compile the real code
COPY ./config.default.toml .
COPY src src
RUN cargo install --locked --path . --root /usr/local

FROM debian:stable-slim

RUN apt-get update && apt-get install -y libssl3 libsasl2-2
ENV ICONOCLAST_LOGGING=json
RUN adduser --system iconoclast
COPY --from=builder /usr/local/bin/iconoclast /usr/local/bin

USER iconoclast
WORKDIR /home/iconoclast

CMD [ "iconoclast" ]
