use chrono::Utc;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AdminAccessToken {
    pub token: String,
    // TODO return Vec<String> and fix table
    pub scopes: String,
    pub expires_at: chrono::DateTime<Utc>,
    pub user_id: uuid::Uuid,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AdminSession {
    pub expires_at: chrono::DateTime<Utc>,
    pub token: String,
    pub user_id: Uuid,
}

impl AdminAccessToken {
    // https://www.oauth.com/oauth2-servers/access-tokens/access-token-lifetime/
    // pub fn lifetime() -> chrono::Duration {
    //     chrono::Duration::days(1)
    // }
}
