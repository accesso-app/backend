-- Add up migration script here
ALTER TABLE "users"
    ADD COLUMN "canonical_email" VARCHAR;
UPDATE "users"
SET canonical_email = lower(email);
ALTER TABLE "users"
    ALTER COLUMN "canonical_email" SET NOT NULL;