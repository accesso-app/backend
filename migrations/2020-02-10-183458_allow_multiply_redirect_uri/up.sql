ALTER TABLE "public"."clients" ALTER COLUMN "redirect_uri" SET DATA TYPE text[] USING array["redirect_uri"];
