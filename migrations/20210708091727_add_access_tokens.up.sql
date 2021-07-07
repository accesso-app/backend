-- Add up migration script here
CREATE TABLE "access_tokens"
(
    "client_id"  uuid      NOT NULL REFERENCES clients (id),
    "created_at" timestamp NOT NULL DEFAULT NOW(),
    "token"      varchar   NOT NULL,
    "user_id"    uuid      NOT NULL REFERENCES users (id),
    PRIMARY KEY ("token")
);

CREATE INDEX "access_tokens_client" ON "access_tokens" USING BTREE ("client_id");
CREATE INDEX "access_tokens_user" ON "access_tokens" USING BTREE ("user_id");
