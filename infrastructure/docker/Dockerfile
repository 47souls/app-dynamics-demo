# syntax=docker/dockerfile:1

################################################################################
# Create a stage for building the application.

ARG APP_NAME=rust
FROM amd64/rust:1.74-slim-bullseye AS build
ARG APP_NAME
RUN cargo new $APP_NAME --bin
WORKDIR $APP_NAME

RUN echo "fn main() { println!(\"dummy.rs\"); }" > src/dummy.rs
RUN rm src/main.rs
COPY Cargo.toml Cargo.lock .
RUN mkdir abi
ADD abi ./abi/
ADD src ./src/

RUN sed -i 's#main.rs#dummy.rs#' Cargo.toml

# needed compile and link
RUN apt update
RUN apt install pkg-config -y
RUN apt install libssl-dev
RUN apt-get install libfreetype6-dev -y
RUN /bin/sh -c 'apt-get install -y aptitude'
RUN aptitude install libfontconfig1-dev libfontconfig -y
RUN apt-get update && apt-get install -y git

# Build the application.
# Leverage a cache mount to /usr/local/cargo/registry/
# for downloaded dependencies and a cache mount to /app/target/ for 
# compiled dependencies which will speed up subsequent builds.
# Leverage a bind mount to the src directory to avoid having to copy the
# source code into the container. Once built, copy the executable to an
# output directory before the cache mounted /app/target is unmounted.
RUN --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
cargo build --config net.git-fetch-with-cli=true --target x86_64-unknown-linux-gnu --release
EOF
