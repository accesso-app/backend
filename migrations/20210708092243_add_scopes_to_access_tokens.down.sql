-- Add down migration script here
ALTER TABLE "access_tokens"
    DROP COLUMN "scopes";
