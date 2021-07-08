-- Add down migration script here
DELETE
FROM "access_tokens";

ALTER TABLE "access_tokens"
    DROP COLUMN "expires_at";

ALTER TABLE "access_tokens"
    ADD COLUMN "created_at" timestamptz NOT NULL;
