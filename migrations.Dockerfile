ARG rust_ver=1.54
FROM rust:${rust_ver}-slim

COPY migrations /app/migrations
COPY ./docker-entrypoint.sh /app/entrypoint.sh

WORKDIR /app
RUN cargo install sqlx-cli --no-default-features --features postgres
RUN chmod +x entrypoint.sh

ENTRYPOINT ["/app/entrypoint.sh"]
LABEL version=0.1