use authmenow_db::schema::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Associations,
    Clone,
    Debug,
    Default,
    Deserialize,
    Identifiable,
    Insertable,
    PartialEq,
    Queryable,
    Serialize,
)]
pub struct Client {
    pub id: uuid::Uuid,
    pub redirect_uri: Vec<String>,
    pub secret_key: String,
    pub scopes: Vec<String>,
    pub title: String,
}

impl Client {
    /// Get one client by its uuid
    pub fn find_by_id(
        conn: &PgConnection,
        client_id: uuid::Uuid,
    ) -> Result<Self, diesel::result::Error> {
        use authmenow_db::schema::clients::dsl::*;

        clients.filter(id.eq(client_id)).first(conn)
    }

    pub fn has_redirect_uri(&self, redirect_uri: &str) -> bool {
        self.redirect_uri
            .iter()
            .find(|uri| *uri == redirect_uri)
            .is_some()
    }

    pub fn has_scope(&self, expected_scope: &str) -> bool {
        self.scopes
            .iter()
            .find(|scope| *scope == expected_scope)
            .is_some()
    }
}

/// Token that handles session of a user
#[derive(
    Associations,
    Clone,
    Debug,
    Deserialize,
    Identifiable,
    Insertable,
    PartialEq,
    Queryable,
    Serialize,
)]
#[primary_key(token)]
#[belongs_to(User)]
pub struct SessionToken {
    pub user_id: uuid::Uuid,
    pub token: String,
    pub expires_at: chrono::NaiveDateTime,
}

impl SessionToken {
    pub fn find_by_token(
        conn: &PgConnection,
        requested_token: &str,
    ) -> Result<Self, diesel::result::Error> {
        use authmenow_db::schema::session_tokens::dsl::*;

        session_tokens.filter(token.eq(requested_token)).first(conn)
    }
}

#[derive(
    Associations,
    Clone,
    Debug,
    Default,
    Deserialize,
    Identifiable,
    Insertable,
    PartialEq,
    Queryable,
    Serialize,
)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub username: Option<String>,
    pub password_hash: String,
}

impl User {
    /// If token already expired, not found should be returned
    pub fn find_by_token_actual(
        conn: &PgConnection,
        token: &str,
    ) -> Result<Self, diesel::result::Error> {
        users::table
            .inner_join(session_tokens::table)
            .select(users::all_columns)
            .filter(session_tokens::token.eq(token))
            .filter(session_tokens::expires_at.gt(chrono::Utc::now().naive_utc()))
            .first(conn)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, Identifiable, Insertable, PartialEq, Queryable)]
#[primary_key(confirmation_code)]
pub struct RegistrationRequest {
    pub confirmation_code: String,
    pub email: String,
    pub expires_at: chrono::NaiveDateTime,
}

impl RegistrationRequest {
    /// Creates registration request with default expiring timeout
    /// Default timeout: 1 day
    pub fn new<E>(email: E) -> Self
    where
        E: ToString,
    {
        Self {
            email: email.to_string(),
            confirmation_code: uuid::Uuid::new_v4().to_string(),
            expires_at: chrono::Utc::now().naive_utc() + chrono::Duration::days(1),
        }
    }

    /// Find request by code and check expires
    pub fn find_by_code_actual(
        conn: &PgConnection,
        code: &str,
    ) -> Result<Self, diesel::result::Error> {
        registration_requests::table
            .filter(registration_requests::confirmation_code.eq(code))
            .filter(registration_requests::expires_at.gt(chrono::Utc::now().naive_utc()))
            .first(conn)
    }
}
