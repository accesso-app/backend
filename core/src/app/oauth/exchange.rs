use crate::contracts::{
    AccessTokenRepo, AuthCodeRepo, ClientRepo, EmailNotification, SecureGenerator,
    UnexpectedDatabaseError,
};
use crate::models::AccessToken;
use crate::App;
use chrono::offset::TimeZone;
use validator::Validate;

pub trait OAuthExchange {
    /// https://www.oauth.com/oauth2-servers/access-tokens/authorization-code-request/
    fn oauth_exchange_access_token(
        &mut self,
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

impl<Db, EMail, Gen> OAuthExchange for App<Db, EMail, Gen>
where
    Db: AuthCodeRepo + ClientRepo + AccessTokenRepo,
    Gen: SecureGenerator,
    EMail: EmailNotification,
{
    /// https://www.oauth.com/oauth2-servers/access-tokens/authorization-code-request/
    fn oauth_exchange_access_token(
        &mut self,
        form: ExchangeAccessTokenForm,
    ) -> Result<AccessTokenCreated, ExchangeFailed> {
        form.validate()?;

        let ExchangeAccessTokenForm {
            grant_type,
            code,
            redirect_uri,
            client_id,
            client_secret,
        } = form;

        match grant_type {
            // exchange authorization_code to access_token
            // https://www.oauth.com/oauth2-servers/access-tokens/access-token-response/
            GrantType::AuthorizationCode => {
                let authorization_code = self
                    .db
                    .auth_code_read(code.to_string())?
                    .ok_or(ExchangeFailed::InvalidClient)?;

                if !authorization_code.is_code_correct(&code)
                    || !authorization_code.is_expired()
                    || !authorization_code.is_redirect_same(&redirect_uri)
                {
                    return Err(ExchangeFailed::InvalidGrant);
                }
                let client = self
                    .db
                    .client_find_by_id(authorization_code.client_id)?
                    .ok_or(ExchangeFailed::InvalidClient)?;

                if !client.is_enabled() || !client.is_allowed_secret(&client_id, &client_secret) {
                    return Err(ExchangeFailed::InvalidClient);
                }

                // TODO: Check scopes
                // if !authorization_code.is_same_valid_scopes(&scopes) {
                //     return Err(ExchangeFailed::InvalidScope)
                // }

                // TODO: Check for grant types

                let access_token = AccessToken {
                    client_id: client.id,
                    expires_at: chrono::Utc::now().naive_utc() + AccessToken::lifetime(),
                    token: self.generator.generate_token_long(),
                    user_id: authorization_code.user_id,
                    scopes: authorization_code.scopes.clone(),
                };

                let created = self.db.access_token_create(access_token)?;

                // https://www.oauth.com/oauth2-servers/access-tokens/access-token-response/
                // TODO: add headers Cache-Control and Pragma
                Ok(AccessTokenCreated {
                    access_token: created.token.clone(),
                    token_type: TokenType::Bearer,
                    expires_in: chrono::Utc.from_utc_datetime(&created.expires_at),
                })
            }
        }
    }
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
