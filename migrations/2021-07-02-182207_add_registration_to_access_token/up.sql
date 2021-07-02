DELETE FROM access_tokens;
ALTER TABLE access_tokens ADD COLUMN "registration_id" uuid REFERENCES user_registrations("id") NOT NULL;
ALTER TABLE "access_tokens" DROP COLUMN "client_id";
ALTER TABLE "access_tokens" DROP COLUMN "user_id";
