# Build Stage
FROM rustlang/rust:nightly-alpine as builder

WORKDIR /code
RUN apk update \
    && apk add build-base openssl-dev zlib-dev \
    && rm -rf /var/cache/apk/*
COPY . .
RUN cargo build --release

# Image Stage
FROM alpine:latest

ENV ROCKET_ENV=production
ENV ROCKET_ADDRESS=0.0.0.0 ROCKET_PORT=3666
EXPOSE 3666

COPY --from=builder /code/target/release/safe-client-gateway /usr/local/bin/safe-client-gateway
CMD safe-client-gateway