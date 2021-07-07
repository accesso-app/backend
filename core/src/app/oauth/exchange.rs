use crate::contracts::UnexpectedDatabaseError;

pub trait OAuthExchange {
    /// https://www.oauth.com/oauth2-servers/access-tokens/authorization-code-request/
    fn oauth_exchange_access_token(
        &self,
        form: ExchangeAccessTokenForm,
    ) -> Result<AccessTokenCreated, ExchangeFailed>;
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum GrantType {
    AuthorizationCode,
}

#[derive(Debug, Validate, PartialEq, Eq, Hash)]
pub struct ExchangeAccessTokenForm {
    pub grant_type: GrantType,

    /// This parameter is for the authorization code received from the authorization server
    /// which will be in the query string parameter “code” in this request.
    pub code: String,

    /// If the redirect URL was included in the initial authorization request,<br/>
    /// it must be included in the token request as well, and must be identical.<br/>
    /// Some services support registering multiple redirect URLs, and some require the redirect URL to be specified on each request.<br/>
    /// The redirect URI in the token request must be an exact match of the redirect URI that was used when generating the authorization code.<br/>
    /// The service must reject the request otherwise.
    #[validate(url)]
    pub redirect_uri: String,

    pub client_id: uuid::Uuid,

    pub client_secret: String,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TokenType {
    Bearer,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct AccessTokenCreated {
    pub access_token: String,
    pub token_type: TokenType,
    pub expires_in: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ExchangeFailed {
    InvalidRequest,
    InvalidClient,
    InvalidGrant,
    // UnsupportedGrantType,
    InvalidScope,
    UnauthorizedClient,
    Unexpected,
}

impl From<validator::ValidationErrors> for ExchangeFailed {
    fn from(_: validator::ValidationErrors) -> Self {
        Self::InvalidRequest
    }
}

impl From<UnexpectedDatabaseError> for ExchangeFailed {
    fn from(_: UnexpectedDatabaseError) -> Self {
        ExchangeFailed::Unexpected
    }
}
