-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "clients"
(
    "id"           uuid DEFAULT uuid_generate_v4(),
    "redirect_uri" varchar NOT NULL,
    "secret_key"   varchar NOT NULL,
    "scopes"       text[]  NOT NULL,
    "title"        varchar NOT NULL,
    PRIMARY KEY ("id")
);
