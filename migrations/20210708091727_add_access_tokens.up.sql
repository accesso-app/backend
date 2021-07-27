-- Add up migration script here
CREATE TABLE "access_tokens"
(
    "client_id"  uuid      NOT NULL REFERENCES clients (id) ON DELETE CASCADE,
    "created_at" timestamp NOT NULL DEFAULT now(),
    "token"      varchar   NOT NULL,
    "user_id"    uuid      NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    PRIMARY KEY ("token")
);

CREATE INDEX "access_tokens_client" ON "access_tokens" USING btree ("client_id");
CREATE INDEX "access_tokens_user" ON "access_tokens" USING btree ("user_id");
