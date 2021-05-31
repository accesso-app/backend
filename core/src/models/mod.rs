pub use access_token::*;
pub use client::*;

mod access_token;
mod client;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RegisterRequest {
    pub email: String,
    pub code: String,
    pub expires_at: chrono::NaiveDateTime,
}

impl RegisterRequest {
    pub fn new(email: String, code: String) -> Self {
        Self {
            email,
            code,
            expires_at: chrono::Utc::now().naive_utc() + chrono::Duration::days(1),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub canonical_email: String,
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SessionToken {
    pub user_id: uuid::Uuid,
    pub token: String,
    pub expires_at: chrono::NaiveDateTime,
}
