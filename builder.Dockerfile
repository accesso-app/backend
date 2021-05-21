ARG RUST_VERSION=1.52.1

FROM rust:$RUST_VERSION as build

ARG DIESEL_VERSION=1.4.6

RUN echo "rust-${RUST_VERSION} diesel-$DIESEL_VERSION"
RUN USER=root cargo install diesel_cli --version ${DIESEL_VERSION} --no-default-features --features postgres && \
    mkdir -p /out && cp $(which diesel) /out/

LABEL version=$RUST_VERSION-$DIESEL_VERSION
