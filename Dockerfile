# Build Stage
FROM rustlang/rust:nightly-buster-slim as builder

WORKDIR /app

RUN set -ex; \ 
  apt-get update; \
  apt-get install -y --no-install-recommends \
  pkg-config \
  ca-certificates libssl-dev \
  && rm -rf /var/lib/apt/lists/*

COPY . .
RUN cargo build --release

# Image Stage
FROM debian:buster-slim

RUN useradd rust

WORKDIR "/app"

ENV ROCKET_ENV=production ROCKET_ADDRESS=0.0.0.0 ROCKET_PORT=3666

EXPOSE 3666

RUN set -ex; \ 
  apt-get update; \
  apt-get install -y --no-install-recommends \
  ca-certificates libssl-dev \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/safe-client-gateway ./
RUN chown rust:rust safe-client-gateway

CMD ["./safe-client-gateway"]