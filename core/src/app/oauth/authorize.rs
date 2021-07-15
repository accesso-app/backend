use crate::models::User;
use async_trait::async_trait;

#[async_trait]
pub trait OAuthAuthorize {
    async fn oauth_request_authorize_code(
        &self,
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

#[derive(Debug, thiserror::Error)]
pub enum RequestAuthCodeFailed {
    #[error("Unauthenticated")]
    Unauthenticated,

    /// The request is missing a required parameter, includes an invalid parameter value, or is otherwise malformed.
    #[error(transparent)]
    InvalidRequest(eyre::Report),

    /// If the user denies the authorization request,
    /// the server will redirect the user back to the redirect URL with error=`access_denied`
    /// in the query string, and no code will be present.
    /// It is up to the app to decide what to display to the user at this point.
    #[error("Access denied: {redirect_uri}")]
    AccessDenied {
        redirect_uri: String,
        state: Option<String>,
    },

    /// The client is not authorized to request an authorization code using this method: The redirect_URI of the service either is incorrect or not provided.
    #[error("Unauthorized client")]
    UnauthorizedClient,

    /// The authorization server does not support obtaining an authorization code using this method
    #[error("Unsupported response type: {redirect_uri}")]
    UnsupportedResponseType {
        redirect_uri: String,
        state: Option<String>,
    },

    /// The requested scope is invalid, unknown, or malformed
    #[error("Invalid scope: {redirect_uri}")]
    InvalidScope {
        redirect_uri: String,
        state: Option<String>,
    },

    /// The authorization server encountered an unexpected condition which prevented it from fulfilling the request
    #[error(transparent)]
    ServerError(#[from] eyre::Report),

    /// The authorization server is currently unable to handle the request due to a temporary overloading or maintenance of the server
    #[error("Temporarily unavailable")]
    TemporarilyUnavailable,
}
