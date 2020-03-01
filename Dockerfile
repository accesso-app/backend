FROM docker.pkg.github.com/authmenow/backend/builder:1.41 as build

ENV USER="root"
WORKDIR /app

COPY ./Cargo.lock ./Cargo.toml ./
RUN cargo new public-api --bin --name authmenow-public-api && \
  cargo new db --lib --name authmenow-db

COPY ./public-api/Cargo.toml ./public-api/Cargo.toml
COPY ./db/Cargo.toml ./db/Cargo.toml
RUN cargo build --release

RUN find ./target -type f -name *authmenow* | xargs rm -rf

COPY ./diesel.toml ./diesel.toml
COPY ./migrations ./migrations
COPY ./db ./db
COPY ./public-api ./public-api

ARG CRATE_NAME

# RUN cargo test --release --verbose --package authmenow-$CRATE_NAME

RUN cargo build --release --package authmenow-$CRATE_NAME

# ----------------------------------------------------------------

FROM docker.pkg.github.com/authmenow/backend/start-tools:1

ARG CRATE_NAME

WORKDIR /app

RUN touch .env

COPY --from=build /out/diesel /bin/
COPY --from=build /app/target/release/authmenow-$CRATE_NAME ./server

COPY --from=build /app/migrations ./migrations
COPY --from=build /app/diesel.toml ./
COPY ./docker-entrypoint.sh ./entrypoint.sh

RUN chmod +x entrypoint.sh && chmod +x server

ENTRYPOINT ["/app/entrypoint.sh"]
CMD ["/app/server"]
