table! {
    access_tokens (token) {
        client_id -> Uuid,
        created_at -> Timestamptz,
        token -> Varchar,
        user_id -> Uuid,
        scopes -> Array<Text>,
    }
}

table! {
    authorization_codes (code) {
        client_id -> Uuid,
        code -> Varchar,
        created_at -> Timestamptz,
        redirect_uri -> Varchar,
        scope -> Nullable<Array<Text>>,
        user_id -> Uuid,
    }
}

table! {
    clients (id) {
        id -> Uuid,
        redirect_uri -> Array<Text>,
        secret_key -> Varchar,
        title -> Varchar,
    }
}

table! {
    registration_requests (confirmation_code) {
        confirmation_code -> Varchar,
        email -> Varchar,
        expires_at -> Timestamptz,
    }
}

table! {
    session_tokens (token) {
        user_id -> Uuid,
        token -> Varchar,
        expires_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        password_hash -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
    }
}

joinable!(access_tokens -> clients (client_id));
joinable!(access_tokens -> users (user_id));
joinable!(authorization_codes -> clients (client_id));
joinable!(authorization_codes -> users (user_id));
joinable!(session_tokens -> users (user_id));

allow_tables_to_appear_in_same_query!(
    access_tokens,
    authorization_codes,
    clients,
    registration_requests,
    session_tokens,
    users,
);
