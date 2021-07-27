DELETE
    FROM access_tokens;

ALTER TABLE access_tokens
    ADD COLUMN registration_id uuid NOT NULL,
    ADD CONSTRAINT access_tokens_registration_id_fkey
        FOREIGN KEY (registration_id)
        REFERENCES user_registrations(id)
        ON DELETE CASCADE;


ALTER TABLE access_tokens DROP COLUMN "client_id";

ALTER TABLE access_tokens DROP COLUMN "user_id";