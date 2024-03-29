-- Add up migration script here
CREATE TABLE "session_tokens"
(
    "user_id"    uuid      NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    "token"      varchar   NOT NULL,
    "expires_at" timestamp NOT NULL DEFAULT now(),
    PRIMARY KEY ("token")
);

CREATE INDEX "session_tokens_user_id" ON "session_tokens" USING btree ("user_id");
