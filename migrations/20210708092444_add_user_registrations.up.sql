-- Add up migration script here
CREATE TABLE "user_registrations"
(
    "id"         uuid      NOT NULL DEFAULT uuid_generate_v4(),
    "client_id"  uuid      NOT NULL REFERENCES clients (id) ON DELETE CASCADE,
    "created_at" timestamp NOT NULL DEFAULT now(),
    "user_id"    uuid      NOT NULL REFERENCES users (id) ON DELETE CASCADE
);

CREATE INDEX "user_registrations_client" ON "user_registrations" USING btree ("client_id");
CREATE INDEX "user_registrations_user" ON "user_registrations" USING btree ("user_id");
