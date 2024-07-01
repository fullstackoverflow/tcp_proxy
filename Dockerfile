# our final base
FROM debian:buster-slim

# copy the build artifact from the build stage
COPY target/release/tcp_proxy /usr/local/bin/tcp_proxy