FROM docker.pkg.github.com/accesso-app/backend/builder:1.47.0 as build

ENV USER="root"
WORKDIR /app

COPY ./resources ./resources
COPY ./diesel.toml ./diesel.toml

COPY ./Cargo.lock ./Cargo.toml ./
COPY ./migrations ./migrations
COPY ./db ./db
COPY ./api-public ./api-public
COPY ./api-internal ./api-internal
COPY ./core ./core

ARG API_NAME

RUN cargo test --release --verbose --package accesso-api-$API_NAME

RUN cargo build --release --package accesso-api-$API_NAME

# ----------------------------------------------------------------

FROM docker.pkg.github.com/accesso-app/backend/start-tools:1.1

ARG API_NAME

WORKDIR /app

RUN touch .env

COPY --from=build /out/diesel /bin/
COPY --from=build /app/target/release/accesso-api-$API_NAME ./server

COPY --from=build /app/migrations ./migrations
COPY --from=build /app/diesel.toml ./
COPY ./docker-entrypoint.sh ./entrypoint.sh

RUN chmod +x entrypoint.sh && chmod +x server

ENTRYPOINT ["/app/entrypoint.sh"]
CMD ["/app/server"]
