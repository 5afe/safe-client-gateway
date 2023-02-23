# Build Image
# match with version in rust-toolchain.toml file
FROM rust:1.65.0-slim-buster as builder

RUN set -ex; \ 
  apt-get update; \
  apt-get install -y --no-install-recommends \
  pkg-config ca-certificates libssl-dev \
  && rm -rf /var/lib/apt/lists/*

ENV CC_aarch64_unknown_linux_musl=clang-15
ENV AR_aarch64_unknown_linux_musl=llvm-ar-15
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="$rustflags_self_contained"
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_RUNNER="$qemu_aarch64"

ARG TARGET
ENV USER=root
ENV TARGET_CC=${TARGET}-gcc
ENV TARGET_AR=${TARGET}-ar
WORKDIR "/app"
# Cache dependencies
# We copy the toolchain requirements first. 
# This will make it possible that all the stages after the init can be cached.
COPY rust-toolchain.toml rust-toolchain.toml
RUN cargo init
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
RUN cargo build --release --locked --target=arm-unknown-linux-gnueabihf

COPY . .

ARG VERSION
ARG BUILD_NUMBER
# Remove fingerprint of app to force recompile (without dependency recompile)
RUN rm -rf target/release/.fingerprint/safe-client-gateway*
RUN cargo build --release --locked --target=arm-unknown-linux-gnueabihf

# Runtime Image
FROM debian:buster-slim


WORKDIR "/app"

ENV APP_USER=rust ROCKET_ENV=production ROCKET_ADDRESS=0.0.0.0 ROCKET_PORT=3666
EXPOSE $ROCKET_PORT
RUN useradd $APP_USER

RUN set -ex; \ 
  apt-get update; \
  apt-get install -y --no-install-recommends \
  ca-certificates libssl-dev \
  gcc-arm-linux-gnueabihf qemu-user libc6-dev-armhf-cross \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder --chown=rust:rust /app/target/release/safe-client-gateway ./
CMD ["./safe-client-gateway"]
