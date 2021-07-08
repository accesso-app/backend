-- Add up migration script here
DROP INDEX "users_username";

ALTER TABLE "users"
    ALTER COLUMN "username" SET NOT NULL;

ALTER TABLE "users"
    RENAME COLUMN "username" TO "first_name";

ALTER TABLE "users"
    ADD COLUMN "last_name" varchar NOT NULL
