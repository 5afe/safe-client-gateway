# Build Stage
FROM rustlang/rust:nightly-buster-slim as builder

WORKDIR /app

RUN set -ex; \ 
  apt-get update; \
  apt-get install -y --no-install-recommends \
  pkg-config ca-certificates libssl-dev \
  && rm -rf /var/lib/apt/lists/*

COPY . .
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
