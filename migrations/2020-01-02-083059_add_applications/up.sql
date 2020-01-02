CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE "applications" (
  "id" uuid DEFAULT uuid_generate_v4(),
  "title" varchar NOT NULL,
  "secret_key" varchar NOT NULL,
  "redirect_uri" varchar NOT NULL,
  "domain" varchar NOT NULL,
  PRIMARY KEY ("id")
);
