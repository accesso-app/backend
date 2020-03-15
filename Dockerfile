FROM docker.pkg.github.com/authmenow/backend/builder:1.41 as build

ENV USER="root"
WORKDIR /app

COPY ./resources ./resources
COPY ./diesel.toml ./diesel.toml

COPY ./Cargo.lock ./Cargo.toml ./
COPY ./migrations ./migrations
COPY ./db ./db
COPY ./public-api ./public-api
COPY ./public-app ./public-app

ARG CRATE_NAME

# RUN cargo test --release --verbose --package authmenow-$CRATE_NAME

RUN cargo build --release --package authmenow-$CRATE_NAME

# ----------------------------------------------------------------

FROM docker.pkg.github.com/authmenow/backend/start-tools:1.0

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
