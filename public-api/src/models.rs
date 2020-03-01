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
    pub first_name: String,
    pub password_hash: String,
    pub last_name: String,
}

static HARDCODED_SALT: &'static str = "AUTHMENOW_SALT";

impl User {
    /// Just creates empty user with generated uuid
    pub fn new() -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            email: String::new(),
            first_name: String::new(),
            last_name: String::new(),
            password_hash: String::new(),
        }
    }

    pub fn password_set(mut self, password: &str) -> Self {
        log::warn!(
            "Password hashed with HARDCODED_SALT. Salt should be passed throught env params"
        );
        let hashed = crate::secure::password_hash(password, HARDCODED_SALT);
        self.password_hash = hashed;
        self
    }

    pub fn password_compare(&mut self, password: &str) -> bool {
        log::warn!(
            "Password hashed with HARDCODED_SALT. Salt should be passed throught env params"
        );
        let hashed = crate::secure::password_hash(password, HARDCODED_SALT);
        self.password_hash == hashed
    }

    pub fn email_set(mut self, email: &str) -> Self {
        self.email = email.to_owned();
        self
    }

    pub fn name_set(mut self, first: &str, last: &str) -> Self {
        self.first_name = first.to_owned();
        self.last_name = last.to_owned();
        self
    }

    pub fn create(&self, conn: &PgConnection) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(users::table)
            .values(self)
            .get_result(conn)
    }

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

    /// Return `true` if email is already registered
    pub fn has_with_email(conn: &PgConnection, email: &str) -> bool {
        users::table
            .filter(users::email.eq(email))
            .count()
            .get_result::<i64>(conn)
            .unwrap_or(0)
            > 0
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
    /// Default timeout: 1 day from today
    pub fn new<E>(email: E) -> Self
    where
        E: ToString,
    {
        Self {
            email: email.to_string(),
            confirmation_code: crate::secure::create_words_password(4, "-"),
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

    pub fn create(self, conn: &PgConnection) -> Result<Self, diesel::result::Error> {
        diesel::insert_into(registration_requests::table)
            .values(&self)
            .get_result(conn)
    }

    pub fn delete_all_for_email(
        conn: &PgConnection,
        email: &str,
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(registration_requests::table)
            .filter(registration_requests::email.eq(email))
            .execute(conn)
    }
}
