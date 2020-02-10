ALTER TABLE "public"."clients" ALTER COLUMN "redirect_uri" SET DATA TYPE varchar USING COALESCE("redirect_uri"[1], '');
