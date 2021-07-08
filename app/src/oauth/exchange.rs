use crate::{App, Service};
use accesso_core::app::oauth::exchange::{
    AccessTokenCreated, ExchangeAccessTokenForm, ExchangeFailed, GrantType, OAuthExchange,
    TokenType,
};
use accesso_core::contracts::{Repository, SecureGenerator};
use accesso_core::models::AccessToken;

use accesso_db::chrono;
use async_trait::async_trait;
use validator::Validate;

#[async_trait]
impl OAuthExchange for App {
    /// https://www.oauth.com/oauth2-servers/access-tokens/authorization-code-request/
    async fn oauth_exchange_access_token(
        &self,
        form: ExchangeAccessTokenForm,
    ) -> Result<AccessTokenCreated, ExchangeFailed> {
        let db = self.get::<Service<dyn Repository>>().unwrap();
        let generator = self.get::<Service<dyn SecureGenerator>>().unwrap();
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
                let authorization_code = db
                    .auth_code_read(code.to_string())
                    .await?
                    .ok_or(ExchangeFailed::InvalidClient)?;

                if !authorization_code.is_code_correct(&code)
                    || !authorization_code.is_expired()
                    || !authorization_code.is_redirect_same(&redirect_uri)
                {
                    return Err(ExchangeFailed::InvalidGrant);
                }
                let client = db
                    .client_find_by_id(authorization_code.client_id)
                    .await?
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
                    expires_at: chrono::Utc::now() + AccessToken::lifetime(),
                    token: generator.generate_token_long(),
                    user_id: authorization_code.user_id,
                    scopes: authorization_code.scopes,
                };

                let created = db.access_token_create(access_token).await?;

                // https://www.oauth.com/oauth2-servers/access-tokens/access-token-response/
                // TODO: add headers Cache-Control and Pragma
                Ok(AccessTokenCreated {
                    access_token: created.token.clone(),
                    token_type: TokenType::Bearer,
                    expires_in: created.expires_at.clone(),
                })
            }
        }
    }
}
