-- Add up migration script here
CREATE TABLE "user_registrations"
(
    "id"         uuid      NOT NULL DEFAULT uuid_generate_v4(),
    "client_id"  uuid      NOT NULL REFERENCES clients (id),
    "created_at" timestamp NOT NULL DEFAULT NOW(),
    "user_id"    uuid      NOT NULL REFERENCES users (id)
);

CREATE INDEX "user_registrations_client" ON "user_registrations" USING BTREE ("client_id");
CREATE INDEX "user_registrations_user" ON "user_registrations" USING BTREE ("user_id");
