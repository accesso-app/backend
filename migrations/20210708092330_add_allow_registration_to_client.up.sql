-- Add up migration script here
ALTER TABLE "clients"
    ADD COLUMN "allowed_registrations" BOOLEAN NOT NULL DEFAULT TRUE;
