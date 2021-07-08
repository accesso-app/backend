-- Add down migration script here
ALTER TABLE "clients"
    DROP COLUMN "allowed_registrations";
