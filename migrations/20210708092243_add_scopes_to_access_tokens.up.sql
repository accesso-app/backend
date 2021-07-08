-- Add up migration script here
ALTER TABLE "access_tokens"
    ADD COLUMN "scopes" text[];

UPDATE "access_tokens"
SET scopes = '{}';

ALTER TABLE "access_tokens"
    ALTER COLUMN "scopes" SET NOT NULL;
