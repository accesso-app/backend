CREATE TABLE "admin_session_tokens"
(
    "user_id"    uuid      NOT NULL REFERENCES users (id),
    "token"      varchar   NOT NULL,
    "expires_at" timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY ("token")
);

CREATE INDEX "admin_session_tokens_user_id" ON "admin_session_tokens" USING btree ("user_id");
