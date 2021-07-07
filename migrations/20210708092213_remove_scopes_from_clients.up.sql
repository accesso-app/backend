-- Add up migration script here
ALTER TABLE "clients"
    DROP COLUMN "scopes";
