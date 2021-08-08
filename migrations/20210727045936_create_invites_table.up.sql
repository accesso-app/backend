CREATE TABLE "application_invites"
(
    "invite"        varchar     NOT NULL,
    "created_at"    timestamptz NOT NULL DEFAULT now(),
    "user_id"       uuid        REFERENCES users (id) ON DELETE CASCADE,
    "registered_at" timestamptz,
    PRIMARY KEY ("invite")
);

CREATE INDEX "application_invites_user_id" ON "application_invites" USING btree("user_id");
