-- Add down migration script here
ALTER TABLE "user_registrations"
    DROP CONSTRAINT user_registrations_pkey;
