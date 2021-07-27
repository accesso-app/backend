-- Add up migration script here
INSERT INTO clients (id, redirect_uri, secret_key, title)
VALUES ('00000000-0000-4000-acce-000000009200',
        '{http://localhost:9000/accesso/done}',
        'CardboxDev',
        'Cardbox[Dev]');