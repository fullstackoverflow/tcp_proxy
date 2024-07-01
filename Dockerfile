# our final base
FROM debian:bookworm-slim

# copy the build artifact from the build stage
COPY target/release/tcp_proxy /usr/local/bin/tcp_proxy