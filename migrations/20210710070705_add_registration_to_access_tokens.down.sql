ALTER TABLE access_tokens
    ADD client_id uuid NOT NULL;

ALTER TABLE access_tokens
    ADD user_id uuid NOT NULL;

ALTER TABLE access_tokens
    DROP COLUMN registration_id;

DELETE
    FROM access_tokens;
