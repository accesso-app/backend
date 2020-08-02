#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AccessToken {
    pub client_id: uuid::Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub token: String,
    pub user_id: uuid::Uuid,
    pub scopes: Vec<String>,
}

impl AccessToken {
    /// https://www.oauth.com/oauth2-servers/access-tokens/access-token-lifetime/
    pub fn expires_at(&self) -> chrono::NaiveDateTime {
        let lifetime = time::Duration::days(1);
        self.created_at + lifetime
    }
}
