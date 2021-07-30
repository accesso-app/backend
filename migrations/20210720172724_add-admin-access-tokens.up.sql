CREATE TABLE admins_access_tokens
(
    token           varchar     NOT NULL,
    scopes          text        NOT NULL,
    expires_at      timestamptz NOT NULL,
    user_id uuid        NOT NULL,
    CONSTRAINT admins_access_tokens_user_id_fkey FOREIGN KEY (user_id) REFERENCES users (id),
    PRIMARY KEY (token)
);
