FROM docker.pkg.github.com/accesso-app/backend/builder:1.45.2 as build

ENV USER="root"
WORKDIR /app

COPY ./resources ./resources
COPY ./diesel.toml ./diesel.toml

COPY ./Cargo.lock ./Cargo.toml ./
COPY ./migrations ./migrations
COPY ./db ./db
COPY ./api-public ./api-public
COPY ./core ./core

ARG CRATE_NAME

RUN cargo test --release --verbose --package accesso-$CRATE_NAME

RUN cargo build --release --package accesso-$CRATE_NAME

# ----------------------------------------------------------------

FROM docker.pkg.github.com/accesso-app/backend/start-tools:1.1

ARG CRATE_NAME

WORKDIR /app

RUN touch .env

COPY --from=build /out/diesel /bin/
COPY --from=build /app/target/release/accesso-$CRATE_NAME ./server

COPY --from=build /app/migrations ./migrations
COPY --from=build /app/diesel.toml ./
COPY ./docker-entrypoint.sh ./entrypoint.sh

RUN chmod +x entrypoint.sh && chmod +x server

ENTRYPOINT ["/app/entrypoint.sh"]
CMD ["/app/server"]
