#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AccessToken {
    pub token: String,
    pub scopes: Vec<String>,
    pub expires_at: chrono::NaiveDateTime,
    pub registration_id: uuid::Uuid,
}

impl AccessToken {
    /// https://www.oauth.com/oauth2-servers/access-tokens/access-token-lifetime/
    pub fn lifetime() -> chrono::Duration {
        chrono::Duration::days(1)
    }
}
