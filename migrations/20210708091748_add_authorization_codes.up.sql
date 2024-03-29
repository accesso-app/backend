-- Add up migration script here
CREATE TABLE "authorization_codes"
(
    "client_id"    uuid      NOT NULL REFERENCES clients (id) ON DELETE CASCADE,
    "code"         varchar   NOT NULL,
    "created_at"   timestamp NOT NULL DEFAULT now(),
    "redirect_uri" varchar   NOT NULL,
    "scope"        text[],
    "user_id"      uuid      NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    PRIMARY KEY ("code")
);

CREATE INDEX "authorization_codes_client" ON "authorization_codes" USING btree ("client_id");
CREATE INDEX "authorization_codes_user" ON "authorization_codes" USING btree ("user_id");
