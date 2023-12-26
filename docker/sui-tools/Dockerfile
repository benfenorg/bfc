# Build application
#
# Copy in all crates, Cargo.toml and Cargo.lock unmodified,
# and build the application.
FROM rust:1.70.0 AS builder
ARG PROFILE=release
ARG GIT_REVISION
ENV GIT_REVISION=$GIT_REVISION
WORKDIR "$WORKDIR/bfc"
RUN apt-get update && apt-get install -y cmake clang

COPY Cargo.toml Cargo.lock ./
COPY crates crates
COPY sui-execution sui-execution
COPY narwhal narwhal
COPY external-crates external-crates

RUN cargo build --profile ${PROFILE} \
    --bin bfc-node \
    --bin bfc \
    --bin bfc-faucet \
    --bin stress \
    --bin bfc-cluster-test

# Production Image
FROM debian:bullseye-slim AS runtime
WORKDIR "$WORKDIR/bfc"
COPY --from=builder /bfc/target/release/bfc-node /usr/local/bin
COPY --from=builder /bfc/target/release/bfc /usr/local/bin
COPY --from=builder /bfc/target/release/bfc-faucet /usr/local/bin
COPY --from=builder /bfc/target/release/stress /usr/local/bin
COPY --from=builder /bfc/target/release/bfc-cluster-test /usr/local/bin

ARG BUILD_DATE
ARG GIT_REVISION
LABEL build-date=$BUILD_DATE
LABEL git-revision=$GIT_REVISION
