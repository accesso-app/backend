# Readme

[![StackShare](http://img.shields.io/badge/tech-stack-0690fa.svg?style=flat)](https://stackshare.io/authmenow/backend) ![API Docker Image CI](https://github.com/accesso-app/backend/workflows/API%20Docker%20Image%20CI/badge.svg)

## Directories and crates

- `db` — database schema, can be reused in different crates
- `core` — main crate with business-logic of the accesso
- `api-private` — crate with actix-web http2 routes, used only inside private network
- `api-internal` — crate with http server, used only by accesso frontend
- `api-admin` — crate with http server, used only by accesso admin frontend
- `api-public` — crate with http server, used from outside

## Local installation

1. Install stable rust (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
2. Install PostgreSQL (`brew install postgresql`)
3. Install diesel_cli (`cargo install diesel_cli --no-default-features --features postgres`)
4. Create database, role and grant all privileges (https://howtocards.io/open/26)
5. Create UUID extension at accesso database (`create extension "uuid-ossp";`)
6. Copy `.env.sample` to `.env` (`cp .env.sample .env`)
7. Migrate database (`diesel migration run`)
8. Run (example: `cd api-public && cargo run`)

## ENVs

- `ACCESSO_MODE` (`"development"`, `"test"`, or `"production"`) — changes environment for config loading
- `DATABASE_URL` — Database connection URL (`postgres://accesso:accesso@localhost:5432/accesso`)
- `ACCESSO_SERVER__PORT` — port to listen on
- `ACCESSO_SERVER__HOST` — host to listen on
- Each variable from [`config/default.toml`](/config/default.toml) can be set via environment variable using [`config`](https://docs.rs/config)

> Note: each variable should be prefixed via "ACCESSO_", section name should be separated with `__`
> Example: server.port -> ACCESSO_SERVER__PORT
> Example: database.pool_size -> ACCESSO_DATABASE__POOL_SIZE

## Configuration

Each API can load configuration in `toml`, `hjson`, `json`, `yaml`, and `ini`. File `config/default.toml` is required.

Config loading formats: `config/{API}-{ENV}` and `.config-{API}-{ENV}`, `API` and `ENV` are optional.

After loading `config/default.toml`, server is trying to load environment configs and specific for API. For example, you have set `ACCESSO_MODE=production` and starting `api-public`:
- `config/default-production.toml`
- `config/public.toml`
- `config/public-production.toml`
- `.config.toml`
- `.config-production.toml`
- `.config-public.toml`
- `.config-public-production.toml`

Configs in repository's root should be prefixed with dot (ex.: `.config-production.json`) and should NOT be committed. It is just local configs.

## Development

- Use [`just`](https://github.com/casey/just) to run commands from [`justfile`](./justfile)
- `just run` — to build and start `api-public` crate (aliased to `just run public`)

## Flows

## Glossary

It's implements simplified OAuth 2.0 flow ([example](https://itnext.io/an-oauth-2-0-introduction-for-beginners-6e386b19f7a9))

- Application — OAuth Client App
- User — the person who wants to be authenticated, to access protected information.
- Accesso — Authorization server

### Authorization flow

Client side:

1. User wants to login. Open https://application/login
2. Application (redirects|opens a window) to https://accesso/session?application_id&redirect_uri&state
3. Accesso checks application request (application_id matches redirect_uri)
4. Accesso shows login form
5. User inserts credentials
6. Accesso checks credentials
7. Accesso sends authorization_code to redirect_uri

Server side:

8. Application sends authorization_code, application_id and secret_key to Accesso
9. Accesso checks authorization_code (application_id matches secret_key, matches authorization_code)
10. Accesso sends access_token back to Application

11. Application makes request using access_token to Accesso to get info about session
12. Accesso checks access_token
13. Accesso returns info about session back to Application
