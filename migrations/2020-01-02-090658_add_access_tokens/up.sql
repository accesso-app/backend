CREATE TABLE "access_tokens" (
  "application_id" uuid NOT NULL REFERENCES applications(id),
  "blocked" bool NOT NULL DEFAULT FALSE,
  "created_at" timestamp NOT NULL DEFAULT NOW(),
  "token" varchar NOT NULL,
  "user_id" uuid NOT NULL REFERENCES users(id),
   PRIMARY KEY ("token")
);
CREATE INDEX "access_tokens_blocked" ON "access_tokens" USING BTREE ("blocked");
CREATE INDEX "access_tokens_application" ON "access_tokens" USING BTREE ("application_id");
CREATE INDEX "access_tokens_user" ON "access_tokens" USING BTREE ("user_id");
