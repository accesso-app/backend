-- change timezone at postgresql.conf
ALTER TABLE "public"."authorization_codes" ALTER COLUMN "created_at" SET DATA TYPE timestamp;
ALTER TABLE "public"."session_tokens" ALTER COLUMN "expires_at" SET DATA TYPE timestamp;
ALTER TABLE "public"."access_tokens" ALTER COLUMN "created_at" SET DATA TYPE timestamp;
