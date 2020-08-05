#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AccessToken {
    pub client_id: uuid::Uuid,
    pub token: String,
    pub user_id: uuid::Uuid,
    pub scopes: Vec<String>,
    pub expires_at: chrono::NaiveDateTime,
}

impl AccessToken {
    /// https://www.oauth.com/oauth2-servers/access-tokens/access-token-lifetime/
    pub fn lifetime() -> time::Duration {
        time::Duration::days(1)
    }
}
