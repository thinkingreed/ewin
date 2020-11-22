# -*- mode: dockerfile -*-
#
# An example Dockerfile showing how to build a Rust executable using this
# image, and deploy it with a tiny Alpine Linux container.

# You can override this `--build-arg BASE_IMAGE=...` to use different
# version of Rust or OpenSSL.
ARG BASE_IMAGE=ekidd/rust-musl-builder:latest

# Our first FROM statement declares the build environment.
FROM ${BASE_IMAGE} AS builder

# Add our source code.
ADD --chown=rust:rust . ./

# Build our application.
RUN apt-get update && apt-get upgrade -y \
    && apt-get install -y  \
    python3  


RUN set -x &&\
    : "Cloning git from ripgrep repo" && \
    git clone https://github.com/BurntSushi/ripgrep && \
    : "Buildng ripgrep" && \
    cd ripgrep && \
    cargo build --release && \
    : "Show version" && \
    cd ./target/x86_64-unknown-linux-musl/release && \
    ./rg --version

# Now, we need to build our _real_ Docker container, copying in `using-sqlx`.
FROM alpine:latest
RUN apk --no-cache add ca-certificates
