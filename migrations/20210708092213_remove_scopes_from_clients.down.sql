-- Add down migration script here
ALTER TABLE "clients"
    ADD COLUMN "scopes" text[];

UPDATE "clients"
SET scopes = '{}';

ALTER TABLE "clients"
    ALTER COLUMN "scopes" SET NOT NULL;
