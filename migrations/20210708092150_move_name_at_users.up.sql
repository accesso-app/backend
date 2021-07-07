-- Add up migration script here
ALTER TABLE "users"
    RENAME COLUMN "first_name" TO "old_first_name";
ALTER TABLE "users"
    RENAME COLUMN "last_name" TO "old_last_name";

ALTER TABLE "users"
    ADD COLUMN "first_name" varchar;
ALTER TABLE "users"
    ADD COLUMN "last_name" varchar;

UPDATE "users"
SET first_name = old_first_name;
UPDATE "users"
SET last_name = old_last_name;

ALTER TABLE "users"
    ALTER COLUMN "first_name" SET NOT NULL;
ALTER TABLE "users"
    ALTER COLUMN "last_name" SET NOT NULL;

ALTER TABLE "users"
    DROP COLUMN "old_first_name";
ALTER TABLE "users"
    DROP COLUMN "old_last_name";
