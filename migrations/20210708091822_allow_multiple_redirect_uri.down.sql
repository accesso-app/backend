-- Add down migration script here
ALTER TABLE "public"."clients"
    ALTER COLUMN "redirect_uri" SET DATA TYPE varchar USING coalesce("redirect_uri"[1], '');
