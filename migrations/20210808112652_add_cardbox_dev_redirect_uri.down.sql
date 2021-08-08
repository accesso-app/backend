-- Add down migration script here
UPDATE clients
SET (id, redirect_uri, secret_key, title) = ('00000000-0000-4000-acce-000000009200',
                                                                    '{http://localhost:9000/accesso/done}',
                                                                    'CardboxDev',
                                                                    'Cardbox[Dev]')
WHERE id = '00000000-0000-4000-acce-000000009100';