FROM docker.pkg.github.com/accesso-app/backend/builder:1.53.0-1.4.1 as build

ENV USER="root"
WORKDIR /app

COPY ./resources ./resources
COPY ./diesel.toml ./diesel.toml

COPY ./Cargo.lock ./Cargo.toml ./
COPY ./migrations ./migrations
COPY ./db ./db
COPY ./settings ./settings
COPY ./api-admin ./api-admin
COPY ./api-public ./api-public
COPY ./api-internal ./api-internal
COPY ./core ./core
COPY ./app ./app

ARG API_NAME

RUN cargo test --release --verbose --package accesso-api-$API_NAME

RUN cargo build --release --package accesso-api-$API_NAME

# ----------------------------------------------------------------

FROM docker.pkg.github.com/accesso-app/backend/start-tools:1.3

ARG API_NAME

WORKDIR /app

RUN touch .env

COPY --from=build /out/diesel /bin/
COPY --from=build /app/target/release/accesso-api-$API_NAME ./server

COPY --from=build /app/migrations ./migrations
COPY --from=build /app/diesel.toml ./
COPY ./config ./config
COPY ./docker-entrypoint.sh ./entrypoint.sh

RUN chmod +x entrypoint.sh && chmod +x server

ENTRYPOINT ["/app/entrypoint.sh"]
CMD ["/app/server"]
