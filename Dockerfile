# Build Stage
# We use a stable rust image as we will switch to nightly via the toolchain file.
FROM rust:1.52.0-slim-buster as builder

RUN set -ex; \ 
  apt-get update; \
  apt-get install -y --no-install-recommends \
  pkg-config ca-certificates libssl-dev \
  && rm -rf /var/lib/apt/lists/*

ENV USER=root
WORKDIR "/app"
# Cache dependencies
# We copy the toolchain requirements first. 
# This will make it possible that all the stages after the init can be cached.
COPY rust-toolchain rust-toolchain
RUN cargo init
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
RUN cargo build --release --locked

COPY . .

ARG VERSION
ARG BUILD_NUMBER
# Remove fingerprint of app to force recompile (without dependency recompile)
RUN rm -rf target/release/.fingerprint/safe-client-gateway*
RUN cargo build --release --locked

# Image Stage
FROM debian:buster-slim


WORKDIR "/app"

ENV APP_USER=rust ROCKET_ENV=production ROCKET_ADDRESS=0.0.0.0 ROCKET_PORT=3666
EXPOSE $ROCKET_PORT
RUN useradd $APP_USER

RUN set -ex; \ 
  apt-get update; \
  apt-get install -y --no-install-recommends \
  ca-certificates libssl-dev \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder --chown=rust:rust /app/target/release/safe-client-gateway ./
CMD ["./safe-client-gateway"]
