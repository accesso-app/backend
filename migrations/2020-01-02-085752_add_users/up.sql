CREATE TABLE "users" (
  "id" uuid DEFAULT uuid_generate_v4(),
  "email" varchar NOT NULL,
  "username" varchar,
  "password_hash" varchar NOT NULL,
  PRIMARY KEY ("id")
);
CREATE UNIQUE INDEX "users_email" ON "users" USING BTREE ("email");
CREATE UNIQUE INDEX "users_username" ON "users" USING BTREE ("username");
