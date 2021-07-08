-- Add down migration script here
ALTER TABLE "users"
    DROP COLUMN "last_name";

ALTER TABLE "users"
    ALTER COLUMN "first_name" DROP NOT NULL;

ALTER TABLE "users"
    RENAME COLUMN "first_name" TO "username";

CREATE UNIQUE INDEX "users_username" ON "users" USING btree ("username");
