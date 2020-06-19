#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Client {
    pub id: uuid::Uuid,
    pub redirect_uri: Vec<String>,
    pub scopes: Vec<String>,
    pub title: String,
}

impl Client {
    /// Check is that response_type allowed for current client
    pub fn is_allowed_response(&self, response_type: &str) -> bool {
        response_type == "code"
    }

    /// https://www.oauth.com/oauth2-servers/redirect-uris/redirect-uri-registration/
    /// https://www.oauth.com/oauth2-servers/redirect-uris/redirect-uri-validation/
    /// The server should reject any authorization requests with redirect URLs that are not an exact match of a registered URL.
    pub fn is_allowed_redirect(&self, redirect_uri: &str) -> bool {
        self.redirect_uri
            .iter()
            .find(|uri| *uri == redirect_uri)
            .is_some()
    }

    /// Check that scopes is exists in application
    /// https://www.oauth.com/oauth2-servers/scope/
    pub fn all_scopes_allowed(&self, scopes: &Vec<String>) -> bool {
        if scopes.is_empty() {
            true
        } else {
            for scope in scopes {
                if let None = self.scopes.iter().find(|exists| *exists == scope) {
                    return false;
                }
            }

            true
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AuthorizationCode {
    pub client_id: uuid::Uuid,
    pub code: String,
    pub created_at: chrono::NaiveDateTime,
    pub redirect_uri: String,
    pub scopes: Vec<String>,
    pub user_id: uuid::Uuid,
}