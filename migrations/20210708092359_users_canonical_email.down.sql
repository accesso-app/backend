-- Add down migration script here
ALTER TABLE "users"
    DROP COLUMN "canonical_email";