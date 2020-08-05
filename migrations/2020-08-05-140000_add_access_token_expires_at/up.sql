DELETE FROM "access_tokens";
ALTER TABLE "access_tokens" DROP COLUMN "created_at";
ALTER TABLE "access_tokens" ADD COLUMN "expires_at" timestamptz NOT NULL;
