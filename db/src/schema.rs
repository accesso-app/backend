table! {
    access_tokens (token) {
        token -> Varchar,
        scopes -> Array<Text>,
        expires_at -> Timestamptz,
        registration_id -> Uuid,
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
        state -> Nullable<Varchar>,
    }
}

table! {
    clients (id) {
        id -> Uuid,
        redirect_uri -> Array<Text>,
        secret_key -> Varchar,
        title -> Varchar,
        allowed_registrations -> Bool,
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
    user_registrations (id) {
        id -> Uuid,
        client_id -> Uuid,
        created_at -> Timestamp,
        user_id -> Uuid,
    }
}

table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        password_hash -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        canonical_email -> Varchar,
    }
}

joinable!(access_tokens -> user_registrations (registration_id));
joinable!(authorization_codes -> clients (client_id));
joinable!(authorization_codes -> users (user_id));
joinable!(session_tokens -> users (user_id));
joinable!(user_registrations -> clients (client_id));
joinable!(user_registrations -> users (user_id));

allow_tables_to_appear_in_same_query!(
    access_tokens,
    authorization_codes,
    clients,
    registration_requests,
    session_tokens,
    user_registrations,
    users,
);
