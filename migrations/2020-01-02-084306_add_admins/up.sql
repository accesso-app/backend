CREATE TABLE "admins" (
  "id" serial,
  "login" varchar NOT NULL,
  "password_hash" varchar NOT NULL,
  "created_at" timestamp NOT NULL DEFAULT NOW(),
  "updated_at" timestamp NOT NULL DEFAULT NOW(),
  "last_login_at" timestamp,
  "blocked" bool NOT NULL DEFAULT false,
  PRIMARY KEY ("id")
);
CREATE UNIQUE INDEX "admins_login" ON "admins" USING BTREE ("login");
