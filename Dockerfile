################################################################################
# Base image
################################################################################

FROM balenalib/rpi-debian as base

ENV INITSYSTEM=on
ENV DEBIAN_FRONTEND=noninteractive
ENV WIFI_CONNECT_VERSION=v4.4.6

################################################################################
# Tools
################################################################################
FROM base as tools

WORKDIR /tmp

# Install build tools
RUN apt-get -q update && apt-get install -yq --no-install-recommends build-essential curl file

# Install youtube-dl
RUN curl -L https://yt-dl.org/downloads/latest/youtube-dl -o /usr/local/bin/youtube-dl

RUN curl -Ls "https://github.com/balena-io/wifi-connect/releases/download/$WIFI_CONNECT_VERSION/wifi-connect-$WIFI_CONNECT_VERSION-linux-rpi.tar.gz" \
  | tar -xvz -C  /tmp

################################################################################
# Rust image
################################################################################

FROM base as rust

# Install build tools
RUN apt-get -q update && apt-get install -yq --no-install-recommends build-essential curl file

ENV PATH=/root/.cargo/bin:$PATH

# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

################################################################################
# Dependencies
################################################################################

FROM rust as dependencies

WORKDIR /build

# Create new fake project ($USER is needed by `cargo new`)
RUN USER=root cargo new app

WORKDIR /build/app

# Copy real app dependencies
COPY Cargo.* ./

# Build fake project with real dependencies
RUN cargo build --release

# Remove the fake app build artifacts
#
# NOTE If your application name contains `-` (`foo-bar` for example)
# then the correct command to remove build artifacts looks like:
#
# RUN rm -rf target/release/foo-bar target/release/deps/foo_bar-*
#                              ^                           ^
RUN rm -rf target/release/vsmp* target/release/deps/vsmp-*

################################################################################
# Builder
################################################################################

FROM rust as builder

# We do not want to download deps, update registry, ... again
COPY --from=dependencies /root/.cargo /root/.cargo

WORKDIR /build/app

# Copy everything, not just source code
COPY . .

# Update already built deps from dependencies image
COPY --from=dependencies /build/app/target target

# Build real app
RUN cargo build --release

################################################################################
# Final image
################################################################################

FROM base

WORKDIR /app

RUN apt-get -q update && apt-get install -yq --no-install-recommends dnsmasq wireless-tools

# Copy binary from builder image
COPY --from=builder /build/app/target/release/vsmp .

COPY --from=tools /usr/local/bin/youtube-dl /usr/local/bin/youtube-dl
COPY --from=tools /tmp/wifi-connect /usr/local/bin/wifi-connect

COPY ./entrypoint.sh ./entrypoint.sh

RUN chmod +x ./entrypoint.sh

# Launch application
ENTRYPOINT ["./entrypoint.sh"]
CMD ["./vsmp"]
