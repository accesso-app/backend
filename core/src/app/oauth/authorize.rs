use crate::contracts::{
    AuthCodeRepo, ClientRepo, EmailNotification, SecureGenerator, UnexpectedDatabaseError,
};
use crate::models::{AuthorizationCode, User};
use crate::App;
use validator::Validate;

pub trait OAuthAuthorize {
    fn oauth_request_authorize_code(
        &mut self,
        actor: Option<User>,
        form: RequestAuthCode,
    ) -> Result<AuthCodeCreated, RequestAuthCodeFailed>;
}

#[derive(Debug, Clone, Validate, PartialEq, Eq, Hash)]
pub struct RequestAuthCode {
    /// Now can receive `code` only
    pub response_type: String,

    pub client_id: uuid::Uuid,

    #[validate(url)]
    pub redirect_uri: String,

    pub scopes: Vec<String>,

    /// The state parameter serves two functions.
    /// When the user is redirected back to your app, whatever value you include as the state will also be included in the redirect.
    /// This gives your app a chance to persist data between the user being directed to the authorization server and back again,
    /// such as using the state parameter as a session key. This may be used to indicate what action in the app to perform after authorization is complete,
    /// for example, indicating which of your app’s pages to redirect to after authorization. This also serves as a CSRF protection mechanism.
    /// When the user is redirected back to your app, double check that the state value matches what you set it to originally.
    /// This will ensure an attacker can’t intercept the authorization flow.
    pub state: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct AuthCodeCreated {
    pub code: String,
    pub redirect_uri: String,
    pub state: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum RequestAuthCodeFailed {
    Unauthenticated,

    /// The request is missing a required parameter, includes an invalid parameter value, or is otherwise malformed.
    InvalidRequest,

    /// If the user denies the authorization request,
    /// the server will redirect the user back to the redirect URL with error=`access_denied`
    /// in the query string, and no code will be present.
    /// It is up to the app to decide what to display to the user at this point.
    AccessDenied {
        redirect_uri: String,
        state: Option<String>,
    },

    /// The client is not authorized to request an authorization code using this method: The redirect_URI of the service either is incorrect or not provided.
    UnauthorizedClient,

    /// The authorization server does not support obtaining an authorization code using this method
    UnsupportedResponseType {
        redirect_uri: String,
        state: Option<String>,
    },

    /// The requested scope is invalid, unknown, or malformed
    InvalidScope {
        redirect_uri: String,
        state: Option<String>,
    },

    /// The authorization server encountered an unexpected condition which prevented it from fulfilling the request
    ServerError,

    /// The authorization server is currently unable to handle the request due to a temporary overloading or maintenance of the server
    TemporarilyUnavailable,
}

impl<Db, Email, Gen> OAuthAuthorize for App<Db, Email, Gen>
where
    Db: ClientRepo + AuthCodeRepo,
    Gen: SecureGenerator,
    Email: EmailNotification,
{
    fn oauth_request_authorize_code(
        &mut self,
        actor: Option<User>,
        form: RequestAuthCode,
    ) -> Result<AuthCodeCreated, RequestAuthCodeFailed> {
        let actor = actor.ok_or(RequestAuthCodeFailed::Unauthenticated)?;

        form.validate()?;

        let client = self
            .db
            .client_find_by_id(form.client_id)?
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
            code: self.generator.generate_token(),
            created_at: chrono::Utc::now().naive_utc(),
            redirect_uri: form.redirect_uri.clone(),
            scopes: form.scopes.clone(),
            user_id: actor.id,
        };

        let created = self.db.auth_code_create(code)?;

        Ok(AuthCodeCreated {
            code: created.code,
            redirect_uri: created.redirect_uri,
            state: form.state,
        })
    }
}

impl From<validator::ValidationErrors> for RequestAuthCodeFailed {
    fn from(_: validator::ValidationErrors) -> Self {
        Self::InvalidRequest
    }
}

impl From<UnexpectedDatabaseError> for RequestAuthCodeFailed {
    fn from(_: UnexpectedDatabaseError) -> Self {
        RequestAuthCodeFailed::ServerError
    }
}
