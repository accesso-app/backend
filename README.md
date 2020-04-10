# Readme

[![StackShare](http://img.shields.io/badge/tech-stack-0690fa.svg?style=flat)](https://stackshare.io/authmenow/backend)

## Local installation

1. Install stable rust (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
2. Install PostgreSQL (`brew install postgresql`)
3. Install diesel_cli (`cargo install diesel_cli --no-default-features --features postgres`)
4. Create database, role and grant all privileges (https://howtocards.io/open/26)
5. Create UUID extension at authmenow database (`create extension "uuid-ossp";`)
6. Copy `.env.sample` to `.env` (`cp .env.sample .env`)
7. Migrate database (`diesel migration run`)
8. Run (example: `cd public-api && cargo run`)

## ENVs

- `DEV` (`"true"` sets true, otherwise false) - sets cookies secure=false,httpOnly=false
- `DATABASE_URL` — Database connection URL (`postgres://authmenow:authmenow@localhost:5432/authmenow`)
- `LISTEN_PORT` — port to listen on
- `LISTEN_HOST` — host to listen on

## Flows

## Glossary

It's implements simplified OAuth 2.0 flow ([example](https://itnext.io/an-oauth-2-0-introduction-for-beginners-6e386b19f7a9))

- Application — OAuth Client App
- User — the person who wants to be authenticated, to access protected information.
- Authmenow — Authorization server

### Authorization flow

Client side:

1. User wants to login. Open https://application/login
2. Application (redirects|opens a window) to https://authmenow/session?application_id&redirect_uri&state
3. Authmenow checks application request (application_id matches redirect_uri)
4. Authmenow shows login form
5. User inserts credentials
6. Authmenow checks credentials
7. Authmenow sends authorization_code to redirect_uri

Server side:

8. Application sends authorization_code, application_id and secret_key to Authmenow
9. Authmenow checks authorization_code (application_id matches secret_key, matches authorization_code)
10. Authmenow sends access_token back to Application

11. Application makes request using access_token to Authmenow to get info about session
12. Authmenow checks access_token
13. Authmenow returns info about session back to Application
