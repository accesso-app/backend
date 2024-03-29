ARG rust_ver=1.63
FROM rust:${rust_ver}-slim

COPY migrations /app/migrations
COPY ./docker-entrypoint.sh /app/entrypoint.sh

WORKDIR /app

RUN seq 1 8 | xargs -I{} mkdir -p /usr/share/man/man{} && \
    apt update && \
    apt -y install libpq-dev postgresql-client ca-certificates pkg-config libssl-dev && \
    update-ca-certificates && \
    apt clean

RUN cargo install sqlx-cli --version 0.5.7 --no-default-features --features postgres
RUN chmod +x entrypoint.sh

ENTRYPOINT ["/app/entrypoint.sh"]
LABEL version=0.1
