table! {
    access_tokens (token) {
        application_id -> Uuid,
        blocked -> Bool,
        created_at -> Timestamp,
        token -> Varchar,
        user_id -> Uuid,
    }
}

table! {
    admins (id) {
        id -> Int4,
        login -> Varchar,
        password_hash -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        last_login_at -> Nullable<Timestamp>,
        blocked -> Bool,
    }
}

table! {
    applications (id) {
        id -> Uuid,
        title -> Varchar,
        secret_key -> Varchar,
        redirect_uri -> Varchar,
        domain -> Varchar,
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

joinable!(access_tokens -> applications (application_id));
joinable!(access_tokens -> users (user_id));

allow_tables_to_appear_in_same_query!(
    access_tokens,
    admins,
    applications,
    users,
);
