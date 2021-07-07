use crate::{App, Service};
use accesso_core::app::oauth::authorize::{
    AuthCodeCreated, OAuthAuthorize, RequestAuthCode, RequestAuthCodeFailed,
};
use accesso_core::contracts::{Repository, SecureGenerator};
use accesso_core::models::{AuthorizationCode, User};

use accesso_db::chrono;
use async_trait::async_trait;
use validator::Validate;

#[async_trait]
impl OAuthAuthorize for App {
    async fn oauth_request_authorize_code(
        &self,
        actor: Option<User>,
        form: RequestAuthCode,
    ) -> Result<AuthCodeCreated, RequestAuthCodeFailed> {
        let db = self.get::<Service<dyn Repository>>().unwrap();
        let generator = self.get::<Service<dyn SecureGenerator>>().unwrap();
        let actor = actor.ok_or(RequestAuthCodeFailed::Unauthenticated)?;

        form.validate()?;

        let client = db
            .client_find_by_id(form.client_id)
            .await?
            .ok_or(RequestAuthCodeFailed::InvalidRequest)?;

        // TODO: register or login?
        // If user already registered in application, just transaprently check
        // If user not registered, show confirmation dialog

        // TODO: check `client.allowed_registrations` when user registers
        // If not allowed reject authorization request

        if !client.is_allowed_redirect(&form.redirect_uri) {
            return Err(RequestAuthCodeFailed::InvalidRequest);
        }

        if !client.is_allowed_response(&form.response_type) {
            return Err(RequestAuthCodeFailed::UnsupportedResponseType {
                redirect_uri: form.redirect_uri.clone(),
                state: form.state,
            });
        }

        // Check if actor already authorized with client
        // TODO: think about authorize confirmation

        let code = AuthorizationCode {
            client_id: client.id,
            code: generator.generate_token(),
            created_at: chrono::Utc::now(),
            redirect_uri: form.redirect_uri.clone(),
            scopes: form.scopes.clone(),
            user_id: actor.id,
        };

        let created = db.auth_code_create(code).await?;

        Ok(AuthCodeCreated {
            code: created.code,
            redirect_uri: created.redirect_uri,
            state: form.state,
        })
    }
}
