CREATE TABLE "registration_requests" (
  "confirmation_code" varchar NOT NULL,
  "email" varchar NOT NULL,
  "expires_at" timestamptz NOT NULL,
  PRIMARY KEY("confirmation_code")
);

CREATE UNIQUE INDEX "registration_requests_email" ON "registration_requests" USING BTREE ("email");
