alter table access_tokens add client_id uuid not null;
alter table access_tokens add user_id uuid not null;
ALTER TABLE access_tokens DROP COLUMN registration_id;
delete from access_tokens;
