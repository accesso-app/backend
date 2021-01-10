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
- `just` — to build and start `api-internal` crate (aliased to `just internal`)
- `just public` — to build and run `api-public`

## Glossary

It's implements simplified OAuth 2.0 flow ([example](https://itnext.io/an-oauth-2-0-introduction-for-beginners-6e386b19f7a9))

- **Client** — OAuth 2.0 Client App.
- **User — the person who wants to be authenticated, to access protected information.
- **Accesso** — Authorization server.
- **Registration Request** — When User registers on Accesso, server send code of request to his email.
- **Authorization Code** — When user authorizes in Client through OAuth 2.0 authorization code flow, he needs to exchange code to access token.
- **Access Token** — Token that exchanged from authorization code, used to send requests from Client server to Accesso server on behalf of User.
- **Session Token** — Token used to authenticate user on Accesso Frontend, it writes to cookies.

## Authorization code flow

> `accesso.server` is an alias for domain of an Accesso instance

1. On the Client side, user presses "Login with Accesso" button.
1. Client redirects to `accesso.server/oauth/authorize` with Client ID, redirect URI and state.
1. Accesso Frontend shows pages and checks User authentication.
1. If User not authenticated, redirect to login, then redirect back to `/oauth/authorize` with all parameters.
1. Accesso Frontend sends `oauthAuthorize` to Accesso Internal API.
1. Accesso Internal API validates Client ID and parameters.
1. [?] Accesso Internal API checks if User already registered in the Client.
1. [?] If User not registered in the Client, Accesso Internal API returns need_confirmation to Accesso Frontend.
1. [?] Accesso Frontend shows a confirmation window to User.
1. [?] When User clicks "Register" in the confirmation window, Accesso Frontend sends _`oauthRegister` (?)_ to Accesso Internal API with all parameters.
1. [?] Accesso Internal API creates Authorization Code and returns it to Accesso Frontend.
1. Accesso Frontend redirects to redirect URI passed from Client with Authorization Code.
1. Client Server send `oauthToken` to Accesso Public API with authorization code, Client ID and secret to exchange it to Access Token.
1. Accesso Public API validates parameters and returns a new Accesso Token.
1. Client Server send any request to Accesso Public API with Access Token.
