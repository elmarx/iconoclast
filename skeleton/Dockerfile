FROM rust:1.87.0 AS builder
# build iconoclast from source directly
ARG ICONOCLAST_SOURCE=0
WORKDIR /usr/src/service

RUN apt-get update && apt-get install -y libssl-dev libsasl2-dev clang

RUN mkdir -p \
    application/src \
    domain/src \
    errors/src \
    kafka/src \
    main/src \
    repository/src \
    web/src

COPY Cargo.toml Cargo.lock* ./
COPY application/Cargo.toml application/
COPY domain/Cargo.toml domain/
COPY errors/Cargo.toml errors/
COPY kafka/Cargo.toml kafka/
COPY main/Cargo.toml main/
COPY repository/Cargo.toml repository/
COPY web/Cargo.toml web/

# compile all dependencies with a dummy for improved caching
RUN mkdir -p main/src/bin && \
  touch application/src/lib.rs && \
  touch domain/src/lib.rs && \
  touch errors/src/lib.rs && \
  touch kafka/src/lib.rs && \
  touch repository/src/lib.rs && \
  touch web/src/lib.rs && \
  echo "fn main() { println!(\"Dummy\"); }" > main/src/bin/dummy.rs && \
  cargo build --release && \
  rm -rf main/src

# now copy over the real sources
COPY ./config.default.toml .
COPY application application
COPY domain domain
COPY errors errors
COPY kafka kafka
COPY main main
COPY repository repository
COPY web web

# update the timestamps so cargo picks up the actual code
RUN touch application/src/lib.rs domain/src/lib.rs errors/src/lib.rs kafka/src/lib.rs repository/src/lib.rs web/src/lib.rs

# patch workspace file to use iconoclast from source
RUN if [ "$ICONOCLAST_SOURCE" != "0" ]; then echo "[patch.\"crates-io\"]\n\niconoclast = { git = \"https://github.com/elmarx/iconoclast.git\", rev = \"$ICONOCLAST_SOURCE\" }" >> Cargo.toml; fi;

# and finally compile the actual code
RUN cargo install --locked --path main --root /usr/local

FROM debian:stable-slim

RUN apt-get update && apt-get install -y libssl3 libsasl2-2
ENV ICONOCLAST_LOGGING=json
RUN useradd -r iconoclast
COPY --from=builder /usr/local/bin/iconoclastd /usr/local/bin

USER iconoclast
WORKDIR /var/lib/iconoclast

CMD [ "iconoclastd" ]
