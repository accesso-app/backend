#![allow(dead_code)]

pub mod api {
    use actix_swagger::{Answer, Api, Method};
    use actix_web::{
        dev::{AppService, Factory, HttpServiceFactory},
        FromRequest,
    };
    use std::future::Future;

    pub struct AccessoPublicApi {
        api: Api,
    }

    impl AccessoPublicApi {
        pub fn new() -> Self {
            AccessoPublicApi { api: Api::new() }
        }
    }

    impl HttpServiceFactory for AccessoPublicApi {
        fn register(self, config: &mut AppService) {
            self.api.register(config);
        }
    }

    impl AccessoPublicApi {
        pub fn bind_oauth_authorize_request<F, T, R>(mut self, handler: F) -> Self
        where
            F: Factory<T, R, Answer<'static, super::paths::oauth_authorize_request::Response>>,
            T: FromRequest + 'static,
            R: Future<Output = Answer<'static, super::paths::oauth_authorize_request::Response>>
                + 'static,
        {
            self.api = self
                .api
                .bind("/oauth/authorize".to_owned(), Method::POST, handler);
            self
        }

        pub fn bind_oauth_token<F, T, R>(mut self, handler: F) -> Self
        where
            F: Factory<T, R, Answer<'static, super::paths::oauth_token::Response>>,
            T: FromRequest + 'static,
            R: Future<Output = Answer<'static, super::paths::oauth_token::Response>> + 'static,
        {
            self.api = self
                .api
                .bind("/oauth/token".to_owned(), Method::POST, handler);
            self
        }

        pub fn bind_viewer_get<F, T, R>(mut self, handler: F) -> Self
        where
            F: Factory<T, R, super::paths::viewer_get::Answer>,
            T: FromRequest + 'static,
            R: Future<Output = super::paths::viewer_get::Answer> + 'static,
        {
            self.api = self.api.bind("/viewer".to_owned(), Method::GET, handler);
            self
        }

        pub fn bind_register_request<F, T, R>(mut self, handler: F) -> Self
        where
            F: Factory<T, R, Answer<'static, super::paths::register_request::Response>>,
            T: FromRequest + 'static,
            R: Future<Output = Answer<'static, super::paths::register_request::Response>> + 'static,
        {
            self.api = self
                .api
                .bind("/register/request".to_owned(), Method::POST, handler);
            self
        }

        pub fn bind_register_confirmation<F, T, R>(mut self, handler: F) -> Self
        where
            F: Factory<T, R, Answer<'static, super::paths::register_confirmation::Response>>,
            T: FromRequest + 'static,
            R: Future<Output = Answer<'static, super::paths::register_confirmation::Response>>
                + 'static,
        {
            self.api = self
                .api
                .bind("/register/confirmation".to_owned(), Method::POST, handler);
            self
        }

        pub fn bind_session_create<F, T, R>(mut self, handler: F) -> Self
        where
            F: Factory<T, R, Answer<'static, super::paths::session_create::Response>>,
            T: FromRequest + 'static,
            R: Future<Output = Answer<'static, super::paths::session_create::Response>> + 'static,
        {
            self.api = self
                .api
                .bind("/session/create".to_owned(), Method::POST, handler);
            self
        }

        pub fn bind_session_get<F, T, R>(mut self, handler: F) -> Self
        where
            F: Factory<T, R, Answer<'static, super::paths::session_get::Response>>,
            T: FromRequest + 'static,
            R: Future<Output = Answer<'static, super::paths::session_get::Response>> + 'static,
        {
            self.api = self
                .api
                .bind("/session/get".to_owned(), Method::POST, handler);
            self
        }
    }

    impl Default for AccessoPublicApi {
        fn default() -> Self {
            let api = AccessoPublicApi::new();
            // add default handlers to response 501, if handler not binded
            api
        }
    }
}

pub mod components {
    pub mod parameters {
        use actix_web::{FromRequest, HttpRequest};
        use serde::Serialize;

        #[derive(Debug, Serialize, Clone)]
        struct ParseHeaderError {
            error: String,
            message: String,
        }

        fn extract_header(req: &HttpRequest, name: String) -> Result<String, ParseHeaderError> {
            let header_error = ParseHeaderError {
                error: "header_required".to_string(),
                message: format!("header '{}' is required", name),
            };

            let header = req.headers().get(name).ok_or(header_error.clone())?;
            let value = header.to_str().map_err(|_| header_error)?.to_string();
            Ok(value)
        }

        pub struct AccessToken(pub String);

        impl FromRequest for AccessToken {
            type Config = ();
            type Error = actix_web::Error;
            type Future = futures::future::Ready<Result<Self, Self::Error>>;

            #[inline]
            fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
                match extract_header(&req, "X-Access-Token".to_string()) {
                    Ok(value) => futures::future::ok(AccessToken(value)),
                    Err(reason) => match serde_json::to_string(&reason) {
                        Ok(json) => futures::future::err(actix_web::error::ErrorBadRequest(json)),
                        Err(error) => {
                            futures::future::err(actix_web::error::ErrorInternalServerError(error))
                        }
                    },
                }
            }
        }
    }

    pub mod responses {
        use serde::{Deserialize, Serialize};

        /// User authenticated in Accesso
        #[derive(Debug, Serialize, Deserialize)]
        pub struct UserAuthenticated {
            pub username: Option<String>,
            #[serde(rename = "displayName")]
            pub display_name: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct UserAnonymous {}

        /// Registration link sent to email, now user can find out when the link expires
        #[derive(Debug, Serialize, Deserialize)]
        pub struct RegistrationRequestCreated {
            /// UTC Unix TimeStamp when the link expires
            #[serde(rename = "expiresAt")]
            pub expires_at: i64,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum RegisterFailedError {
            #[serde(rename = "email_already_registered")]
            EmailAlreadyRegistered,

            #[serde(rename = "invalid_form")]
            InvalidForm,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RegisterFailed {
            pub error: RegisterFailedError,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum RegisterConfirmationFailedError {
            #[serde(rename = "code_invalid_or_expired")]
            CodeInvalidOrExpired,

            #[serde(rename = "email_already_activated")]
            EmailAlreadyActivated,

            #[serde(rename = "invalid_form")]
            InvalidForm,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RegisterConfirmationFailed {
            pub error: RegisterConfirmationFailedError,
        }

        #[derive(Debug, Serialize)]
        pub struct ViewerGetSuccess {
            #[serde(rename = "firstName")]
            pub first_name: String,

            #[serde(rename = "lastName")]
            pub last_name: String,

            pub id: uuid::Uuid,
        }

        #[derive(Debug, Serialize)]
        pub struct SessionCreateSucceeded {
            #[serde(rename = "firstName")]
            pub first_name: String,

            #[serde(rename = "lastName")]
            pub last_name: String,
        }

        #[doc = "Login failed"]
        #[derive(Debug, Serialize)]
        pub struct SessionCreateFailed {
            pub error: SessionCreateFailedError,
        }

        #[derive(Debug, Serialize)]
        pub enum SessionCreateFailedError {
            #[serde(rename = "invalid_credentials")]
            InvalidCredentials,

            #[serde(rename = "invalid_form")]
            InvalidForm,

            #[serde(rename = "invalid_payload")]
            InvalidPayload,
        }

        #[derive(Debug, Serialize)]
        pub struct SessionGetSuccess {
            pub user: super::schemas::SessionUser,
        }

        #[doc = "Authorization completed, now access token can be obtained."]
        #[derive(Debug, Serialize)]
        pub struct OAuthAuthorizeDone {
            #[doc = "User should be redirected to"]
            #[serde(rename = "redirectUri")]
            pub redirect_uri: String,

            #[doc = "This parameter contains the authorization code which the client will later exchange for an access token."]
            pub code: String,

            #[doc = "If the initial request contained a state parameter, the response must also include the exact value from the request. The client will be using this to associate this response with the initial request."]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub state: Option<String>,
        }

        /// Possible errors:
        /// If the user denies the authorization request, the server will redirect the user back to the redirect URL with error=`access_denied` in the query string, and no code will be present. It is up to the app to decide what to display to the user at this point.
        /// - `invalid_request` — The request is missing a required parameter, includes an invalid parameter value, or is otherwise malformed.
        /// - `unsupported_response_type` — The authorization server does not support obtaining an authorization code using this method.
        /// - `invalid_scope` — The requested scope is invalid, unknown, or malformed.
        /// - `server_error` — The authorization server encountered an unexpected condition which prevented it from fulfilling the request.
        /// - `temporarily_unavailable` — The authorization server is currently unable to handle the request due to a temporary overloading or maintenance of the server.
        /// [OAuth2 Possible Errors](https://www.oauth.com/oauth2-servers/server-side-apps/possible-errors/)
        #[derive(Debug, Serialize)]
        pub enum OAuthAuthorizeRequestFailureError {
            #[serde(rename = "invalid_request")]
            InvalidRequest,

            #[serde(rename = "access_denied")]
            AccessDenied,

            #[serde(rename = "unauthorized_client")]
            UnauthorizedClient,

            #[serde(rename = "unauthenticated_user")]
            UnauthenticatedUser,

            #[serde(rename = "unsupported_response_type")]
            UnsupportedResponseType,

            #[serde(rename = "invalid_scope")]
            InvalidScope,

            #[serde(rename = "server_error")]
            ServerError,

            #[serde(rename = "temporarily_unavailable")]
            TemporarilyUnavailable,
        }

        /// There are two different kinds of errors to handle. The first kind of error is when the developer did something wrong when creating the authorization request. The other kind of error is when the user rejects the request (clicks the “Deny” button).
        /// If there is something wrong with the syntax of the request, such as the redirect_uri or client_id is invalid, then it’s important not to redirect the user and instead you should show the error message directly. This is to avoid letting your authorization server be used as an open redirector.
        /// If the redirect_uri and client_id are both valid, but there is still some other problem, it’s okay to redirect the user back to the redirect URI with the error in the query string.
        #[derive(Debug, Serialize)]
        pub struct OAuthAuthorizeRequestFailure {
            pub error: OAuthAuthorizeRequestFailureError,

            #[doc = "User should be redirected to if passed redirectUri and clientId is correct"]
            #[serde(rename = "redirectUri")]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub redirect_uri: Option<String>,

            #[doc = "If the initial request contained a state parameter, the response must also include the exact value from the request. The client will be using this to associate this response with the initial request."]
            #[serde(skip_serializing_if = "Option::is_none")]
            pub state: Option<String>,
        }

        #[derive(Debug, Serialize)]
        pub enum ViewerGetFailureError {
            #[serde(rename = "invalid_token")]
            InvalidToken,

            #[serde(rename = "unauthorized")]
            Unauthorized,
        }

        #[derive(Debug, Serialize)]
        pub struct ViewerGetFailure {
            pub error: ViewerGetFailureError,
        }

        /// The auth services validated the request and responds with an access token
        /// [OAuth2 Example Flow](https://www.oauth.com/oauth2-servers/server-side-apps/example-flow/)
        #[derive(Debug, Serialize)]
        pub struct OAuthAccessTokenCreated {
            pub access_token: String,
            pub token_type: OAuthAccessTokenCreatedTokenType,

            /// UTC Unix TimeStamp when the access token expires
            pub expires_in: i64,
        }

        #[derive(Debug, Serialize)]
        pub enum OAuthAccessTokenCreatedTokenType {
            #[serde(rename = "bearer")]
            Bearer,
        }

        /// When you can't exchange authorization code to access token
        #[derive(Debug, Serialize)]
        pub struct OAuthAccessTokenFailure {
            pub error: OAuthAccessTokenFailureError,
        }

        #[derive(Debug, Serialize)]
        pub enum OAuthAccessTokenFailureError {
            #[serde(rename = "invalid_request")]
            InvalidRequest,

            #[serde(rename = "invalid_client")]
            InvalidClient,

            #[serde(rename = "invalid_grant")]
            InvalidGrant,

            #[serde(rename = "invalid_scope")]
            InvalidScope,

            #[serde(rename = "unauthorized_client")]
            UnauthorizedClient,

            #[serde(rename = "unsupported_grant_type")]
            UnsupportedGrantType,
        }
    }

    pub mod request_bodies {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Register {
            pub email: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct RegisterConfirmation {
            #[serde(rename = "confirmationCode")]
            pub confirmation_code: String,

            #[serde(rename = "firstName")]
            pub first_name: String,

            #[serde(rename = "lastName")]
            pub last_name: String,

            pub password: String,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct SessionCreate {
            pub email: String,
            pub password: String,
        }

        /// responseType is set to code indicating that you want an authorization code as the response.
        #[derive(Debug, Serialize, Deserialize)]
        pub enum OAuthAuthorizeResponseType {
            #[serde(rename = "code")]
            Code,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct OAuthAuthorize {
            #[doc = "responseType is set to code indicating that you want an authorization code as the response."]
            #[serde(rename = "responseType")]
            pub response_type: OAuthAuthorizeResponseType,

            #[doc = "The clientId is the identifier for your app. You will have received a clientId when first registering your app with the service."]
            #[serde(rename = "clientId")]
            pub client_id: uuid::Uuid,

            /// The redirectUri may be optional depending on the API, but is highly recommended.
            /// This is the URL to which you want the user to be redirected after the authorization is complete.
            /// This must match the redirect URL that you have previously registered with the service.
            #[serde(rename = "redirectUri")]
            pub redirect_uri: String, // implement url::Url deserializer

            /// Include one or more scope values (space-separated) to request additional levels of access.
            /// The values will depend on the particular service.
            /// Example user:view user:edit
            pub scope: Option<String>, // implement custom Scope deserializer

            /// The state parameter serves two functions.
            /// When the user is redirected back to your app, whatever value you include as the state will also be included in the redirect.
            /// This gives your app a chance to persist data between the user being directed to the authorization server and back again,
            /// such as using the state parameter as a session key. This may be used to indicate what action in the app to perform after authorization is complete,
            /// for example, indicating which of your app’s pages to redirect to after authorization. This also serves as a CSRF protection mechanism.
            /// When the user is redirected back to your app, double check that the state value matches what you set it to originally.
            /// This will ensure an attacker can’t intercept the authorization flow.
            pub state: Option<String>,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub enum OAuthAccessTokenExchangeGrantType {
            #[serde(rename = "authorization_code")]
            AuthorizationCode,
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct OAuthAccessTokenExchange {
            pub grant_type: OAuthAccessTokenExchangeGrantType,
            pub code: String,
            pub redirect_uri: String,
            pub client_id: uuid::Uuid,
            pub client_secret: String,
        }
    }

    pub mod schemas {
        use serde::{Deserialize, Serialize};

        #[doc = "Current user in a session"]
        #[derive(Debug, Serialize, Deserialize)]
        pub struct SessionUser {
            #[serde(rename = "firstName")]
            pub first_name: String,

            #[serde(rename = "lastName")]
            pub last_name: String,
        }
    }
}

pub mod paths {
    use super::components::responses;
    pub mod register_request {
        use super::responses;
        use actix_swagger::ContentType;
        use actix_web::http::StatusCode;
        use serde::Serialize;

        pub type Answer = actix_swagger::Answer<'static, Response>;

        #[derive(Debug, Serialize)]
        #[serde(untagged)]
        pub enum Response {
            Created(responses::RegistrationRequestCreated),
            BadRequest(responses::RegisterFailed),
            Unexpected,
        }

        impl Response {
            #[inline]
            pub fn answer<'a>(self) -> actix_swagger::Answer<'a, Self> {
                let status = match self {
                    Self::Created(_) => StatusCode::CREATED,
                    Self::BadRequest(_) => StatusCode::BAD_REQUEST,
                    Self::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
                };

                let content_type = match self {
                    Self::Created(_) => Some(ContentType::Json),
                    Self::BadRequest(_) => Some(ContentType::Json),
                    Self::Unexpected => None,
                };

                actix_swagger::Answer::new(self)
                    .status(status)
                    .content_type(content_type)
            }
        }
    }

    pub mod session_create {
        use super::responses;
        use actix_swagger::{Answer, ContentType};
        use actix_web::http::StatusCode;
        use serde::Serialize;

        #[derive(Debug, Serialize)]
        #[serde(untagged)]
        pub enum Response {
            Created(responses::SessionCreateSucceeded),
            BadRequest(responses::SessionCreateFailed),
            Unexpected,
        }

        impl Response {
            #[inline]
            pub fn answer<'a>(self) -> Answer<'a, Self> {
                let status = match self {
                    Self::Created(_) => StatusCode::CREATED,
                    Self::BadRequest(_) => StatusCode::BAD_REQUEST,
                    Self::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
                };

                let content_type = match self {
                    Self::Created(_) => Some(ContentType::Json),
                    Self::BadRequest(_) => Some(ContentType::Json),
                    Self::Unexpected => None,
                };

                Answer::new(self).status(status).content_type(content_type)
            }
        }
    }

    pub mod viewer_get {
        use super::responses;
        use actix_swagger::ContentType;
        use actix_web::http::StatusCode;
        use serde::Serialize;

        pub type Answer = actix_swagger::Answer<'static, Response>;

        #[derive(Debug, Serialize)]
        #[serde(untagged)]
        pub enum Response {
            Ok(responses::ViewerGetSuccess),
            BadRequest(responses::ViewerGetFailure),
            Unexpected,
        }

        impl Into<Answer> for Response {
            #[inline]
            fn into(self) -> Answer {
                let status = match self {
                    Self::Ok(_) => StatusCode::OK,
                    Self::BadRequest(_) => StatusCode::BAD_REQUEST,
                    Self::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
                };

                let content_type = match self {
                    Self::Ok(_) => Some(ContentType::Json),
                    Self::BadRequest(_) => None,
                    Self::Unexpected => None,
                };

                Answer::new(self).status(status).content_type(content_type)
            }
        }
    }

    pub mod session_get {
        use super::responses;
        use actix_swagger::ContentType;
        use actix_web::http::StatusCode;
        use serde::Serialize;

        pub type Answer = actix_swagger::Answer<'static, Response>;

        #[derive(Debug, Serialize)]
        #[serde(untagged)]
        pub enum Response {
            Ok(responses::SessionGetSuccess),
            Unauthorized,
            Unexpected,
        }

        impl Into<Answer> for Response {
            #[inline]
            fn into(self) -> Answer {
                let status = match self {
                    Self::Ok(_) => StatusCode::OK,
                    Self::Unauthorized => StatusCode::UNAUTHORIZED,
                    Self::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
                };

                let content_type = match self {
                    Self::Ok(_) => Some(ContentType::Json),
                    Self::Unauthorized => None,
                    Self::Unexpected => None,
                };

                Answer::new(self).status(status).content_type(content_type)
            }
        }
    }

    pub mod register_confirmation {
        use super::responses;
        use actix_swagger::{Answer, ContentType};
        use actix_web::http::StatusCode;
        use serde::Serialize;

        #[derive(Debug, Serialize)]
        #[serde(untagged)]
        pub enum Response {
            Created,
            BadRequest(responses::RegisterConfirmationFailed),
            Unexpected,
        }

        impl Response {
            #[inline]
            pub fn answer<'a>(self) -> Answer<'a, Self> {
                let status = match self {
                    Self::Created => StatusCode::CREATED,
                    Self::BadRequest(_) => StatusCode::BAD_REQUEST,
                    Self::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
                };

                let content_type = match self {
                    Self::Created => None,
                    Self::BadRequest(_) => Some(ContentType::Json),
                    Self::Unexpected => None,
                };

                Answer::new(self).status(status).content_type(content_type)
            }
        }
    }

    pub mod oauth_authorize_request {
        use super::responses;
        use actix_swagger::{Answer, ContentType};
        use actix_web::http::StatusCode;
        use serde::Serialize;

        #[derive(Debug, Serialize)]
        #[serde(untagged)]
        pub enum Response {
            Ok(responses::OAuthAuthorizeDone),

            BadRequest(responses::OAuthAuthorizeRequestFailure),

            InternalServerError,
        }

        impl Response {
            #[inline]
            pub fn answer<'a>(self) -> Answer<'a, Self> {
                let status = match self {
                    Self::Ok(_) => StatusCode::OK,
                    Self::BadRequest(_) => StatusCode::BAD_REQUEST,
                    Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
                };

                let content_type = match self {
                    Self::Ok(_) => Some(ContentType::Json),
                    Self::BadRequest(_) => Some(ContentType::Json),
                    Self::InternalServerError => None,
                };

                Answer::new(self).content_type(content_type).status(status)
            }
        }
    }

    pub mod oauth_token {
        use super::responses;
        use actix_swagger::{Answer, ContentType};
        use actix_web::http::StatusCode;
        use serde::Serialize;

        #[derive(Debug, Serialize)]
        #[serde(untagged)]
        pub enum Response {
            Created(responses::OAuthAccessTokenCreated),
            BadRequest(responses::OAuthAccessTokenFailure),
            InternalServerError,
        }

        impl Response {
            #[inline]
            pub fn answer<'a>(self) -> Answer<'a, Self> {
                let status = match self {
                    Self::Created(_) => StatusCode::OK,
                    Self::BadRequest(_) => StatusCode::BAD_REQUEST,
                    Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
                };

                let content_type = match self {
                    Self::Created(_) => Some(ContentType::Json),
                    Self::BadRequest(_) => Some(ContentType::Json),
                    Self::InternalServerError => None,
                };

                Answer::new(self).content_type(content_type).status(status)
            }
        }
    }
}
