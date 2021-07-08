-- Add up migration script here
ALTER TABLE "public"."clients"
    ALTER COLUMN "redirect_uri" SET DATA TYPE text[] USING ARRAY ["redirect_uri"];
