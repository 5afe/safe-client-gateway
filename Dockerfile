# Build Image
# match with version in rust-toolchain.toml file
# BUILDPLATFORM is available inside Docker context. It contains the name of the host platform
# See https://docs.docker.com/build/building/multi-platform/#building-multi-platform-images
# See https://jakewharton.com/cross-compiling-static-rust-binaries-in-docker-for-raspberry-pi/
FROM --platform=$BUILDPLATFORM rust:1.68.1 as builder

# TARGETPLATFORM is available inside the Docker context. It contains the name of the target platform
# See https://docs.docker.com/build/building/multi-platform/#building-multi-platform-images
ARG TARGETPLATFORM
ARG VERSION
ARG BUILD_NUMBER

# This Docker image supports two targets: linux/arm64 and linux/amd64
# In order to install the correct Rust build tools for each target we map the buildx targets to a rust target.
# The mapping result is then written to rust_target.txt
RUN case "$TARGETPLATFORM" in \
      "linux/arm64") echo aarch64-unknown-linux-gnu > /rust_target.txt ;; \
      "linux/amd64") echo x86_64-unknown-linux-gnu > /rust_target.txt ;; \
      *) exit 1 ;; \
    esac

# If the target platform is linux/arm64 we need to additionally configure the Ring crate which is used by this project
# This setup specifies the correct linker to be used when targetting aarch64-unknown-linux-gnu
# See: https://github.com/briansmith/ring/blob/main/BUILDING.md#cross-compiling
RUN if [ "$TARGETPLATFORM" = "linux/arm64" ] ; then \
    export CC_aarch64_unknown_linux_gnu=clang && \
    export AR_aarch64_unknown_linux_gnu=llvm-ar && \
    export CFLAGS_aarch64_unknown_linux_gnu="--sysroot=/usr/aarch64-linux-gnu" && \
    export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc && \
    export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUNNER="qemu-aarch64 -L /usr/aarch64-linux-gnu"; \
    fi

# Install the necessary Rust tooling to build for the target obtained from TARGETPLATFORM
RUN rustup target add $(cat /rust_target.txt)

# If we are targetting linux/arm64 we need to install some additional build tools
# See: https://github.com/briansmith/ring/blob/main/BUILDING.md#cross-compiling
RUN if [ "$TARGETPLATFORM" = "linux/arm64" ] ;  \
    then apt-get update && apt-get install -y \
    clang llvm gcc-aarch64-linux-gnu libc6-dev-arm64-cross && \
    rm -rf /var/lib/apt/lists/*;  \
    fi

WORKDIR "/app"
COPY .cargo ./.cargo
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release --locked --target $(cat /rust_target.txt)
# Copy the project to a target independent location. This will be used by the Runtime image
RUN cp target/$(cat /rust_target.txt)/release/safe-client-gateway .

# Runtime Image
FROM debian:bullseye-slim
WORKDIR "/app"

ENV ROCKET_ENV=production ROCKET_ADDRESS=0.0.0.0 ROCKET_PORT=3666
EXPOSE $ROCKET_PORT

RUN set -ex; \ 
  apt-get update; \
  apt-get install -y --no-install-recommends \
  ca-certificates \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/safe-client-gateway ./
CMD ["./safe-client-gateway"]
