use chrono::Utc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Application {
    pub id: uuid::Uuid,
    pub is_dev: bool,
    pub redirect_uri: Vec<String>,
    pub title: String,
    pub secret_key: String,
    pub allowed_registrations: bool,
}

impl Application {
    /// Check is that response_type allowed for current application
    pub fn is_allowed_response(&self, response_type: &str) -> bool {
        response_type == "code"
    }

    /// https://www.oauth.com/oauth2-servers/redirect-uris/redirect-uri-registration/
    /// https://www.oauth.com/oauth2-servers/redirect-uris/redirect-uri-validation/
    /// The server should reject any authorization requests with redirect URLs that are not an exact match of a registered URL.
    pub fn is_allowed_redirect(&self, redirect_uri: &str) -> bool {
        self.redirect_uri.iter().any(|uri| uri == redirect_uri)
    }

    /// https://www.oauth.com/oauth2-servers/access-tokens/authorization-code-request/
    pub fn is_allowed_secret(&self, id: &uuid::Uuid, secret: &str) -> bool {
        self.id == *id && self.secret_key == secret
    }

    pub fn is_enabled(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AuthorizationCode {
    pub client_id: uuid::Uuid,
    pub code: String,
    pub created_at: chrono::DateTime<Utc>,
    pub redirect_uri: String,
    pub scopes: Vec<String>,
    pub user_id: uuid::Uuid,
}

impl AuthorizationCode {
    /// https://www.oauth.com/oauth2-servers/access-tokens/authorization-code-request/
    pub fn is_redirect_same(&self, redirect_uri: &str) -> bool {
        self.redirect_uri == redirect_uri
    }

    pub fn is_code_correct(&self, code: &str) -> bool {
        self.code == code
    }

    pub fn is_expired(&self) -> bool {
        let lifetime = chrono::Duration::minutes(15);
        let now = chrono::Utc::now();

        (self.created_at + lifetime) > now
    }
}
