-- Add up migration script here
-- change timezone to UTC at postgresql.conf
ALTER TABLE "public"."authorization_codes"
    ALTER COLUMN "created_at" SET DATA TYPE timestamptz;
ALTER TABLE "public"."session_tokens"
    ALTER COLUMN "expires_at" SET DATA TYPE timestamptz;
ALTER TABLE "public"."access_tokens"
    ALTER COLUMN "created_at" SET DATA TYPE timestamptz;
