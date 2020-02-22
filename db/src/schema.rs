table! {
    access_tokens (token) {
        client_id -> Uuid,
        created_at -> Timestamp,
        token -> Varchar,
        user_id -> Uuid,
    }
}

table! {
    authorization_codes (code) {
        client_id -> Uuid,
        code -> Varchar,
        created_at -> Timestamp,
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
        scopes -> Array<Text>,
        title -> Varchar,
    }
}

table! {
    session_tokens (token) {
        user_id -> Uuid,
        token -> Varchar,
        expires_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        username -> Nullable<Varchar>,
        password_hash -> Varchar,
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
    session_tokens,
    users,
);
