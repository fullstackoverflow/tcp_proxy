FROM rust:1.67 as build

# create a new empty shell project
RUN USER=root cargo new --bin tcp_proxy
WORKDIR /tcp_proxy

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/tcp_proxy*
RUN cargo build --release

# our final base
FROM debian:buster-slim

# copy the build artifact from the build stage
COPY --from=build /tcp_proxy/target/release/tcp_proxy /usr/local/bin/tcp_proxy