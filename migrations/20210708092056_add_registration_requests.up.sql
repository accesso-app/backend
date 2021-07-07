-- Add up migration script here
CREATE TABLE "registration_requests"
(
    "confirmation_code" varchar     NOT NULL,
    "email"             varchar     NOT NULL,
    "expires_at"        timestamptz NOT NULL,
    PRIMARY KEY ("confirmation_code")
);

