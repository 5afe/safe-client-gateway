# Build Stage
FROM rustlang/rust:nightly-buster-slim as builder

RUN set -ex; \ 
  apt-get update; \
  apt-get install -y --no-install-recommends \
  pkg-config ca-certificates libssl-dev \
  && rm -rf /var/lib/apt/lists/*

ENV USER=root
WORKDIR "/app"
# Cache dependencies
RUN cargo init
COPY Cargo.toml Cargo.toml
RUN cargo build --release

COPY . .

ARG VERSION
ARG BUILD_NUMBER
# Remove fingerprint of app to force recompile (without dependency recompile)
RUN rm -rf target/release/.fingerprint/safe-client-gateway*
RUN cargo build --release

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

COPY --from=builder --chown=$APP_USER:$APP_USER /app/target/release/safe-client-gateway ./
CMD ["./safe-client-gateway"]
