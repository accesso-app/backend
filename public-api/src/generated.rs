#![allow(dead_code)]

pub mod api {
    use actix_swagger::{Answer, Api};
    use actix_web::{
        dev::{AppService, Factory, HttpServiceFactory},
        http::Method,
        FromRequest,
    };
    use std::future::Future;

    pub struct AuthmenowPublicApi {
        api: Api,
    }

    impl AuthmenowPublicApi {
        pub fn new() -> Self {
            AuthmenowPublicApi { api: Api::new() }
        }
    }

    impl HttpServiceFactory for AuthmenowPublicApi {
        fn register(self, config: &mut AppService) {
            self.api.register(config);
        }
    }

    impl AuthmenowPublicApi {
        pub fn bind_oauth_authorize_request<F, T, R>(mut self, handler: F) -> Self
        where
            F: Factory<T, R, Answer<'static, super::paths::oauth_authorize_request::Response>>,
            T: FromRequest + 'static,
            R: Future<Output = Answer<'static, super::paths::oauth_authorize_request::Response>>
                + 'static,
        {
            self.api = self
                .api
                .bind("/oauth/authorize".to_owned(), Method::GET, handler);
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

        pub fn bind_session_get<F, T, R>(mut self, handler: F) -> Self
        where
            F: Factory<T, R, Answer<'static, super::paths::SessionGetResponse>>,
            T: FromRequest + 'static,
            R: Future<Output = Answer<'static, super::paths::SessionGetResponse>> + 'static,
        {
            self.api = self.api.bind("/session".to_owned(), Method::GET, handler);
            self
        }

        pub fn bind_session_create<F, T, R>(mut self, handler: F) -> Self
        where
            F: Factory<T, R, Answer<'static, super::paths::SessionCreateResponse>>,
            T: FromRequest + 'static,
            R: Future<Output = Answer<'static, super::paths::SessionCreateResponse>> + 'static,
        {
            self.api = self.api.bind("/session".to_owned(), Method::POST, handler);
            self
        }

        pub fn bind_session_delete<F, T, R>(mut self, handler: F) -> Self
        where
            F: Factory<T, R, Answer<'static, super::paths::SessionDeleteResponse>>,
            T: FromRequest + 'static,
            R: Future<Output = Answer<'static, super::paths::SessionDeleteResponse>> + 'static,
        {
            self.api = self
                .api
                .bind("/session".to_owned(), Method::DELETE, handler);
            self
        }
    }

    impl Default for AuthmenowPublicApi {
        fn default() -> Self {
            let api = AuthmenowPublicApi::new();
            // add default handlers to response 501, if handler not binded
            api
        }
    }
}

pub mod components {
    pub mod parameters {
        use serde::{Deserialize, Serialize};

        /// response_type is set to code indicating that you want an authorization code as the response.
        #[derive(Debug, Serialize, Deserialize)]
        pub enum OAuthResponseType {
            #[serde(rename = "code")]
            Code,
        }

        /// The client_id is the identifier for your app.
        /// You will have received a client_id when first registering your app with the service.
        pub type OAuthClientId = uuid::Uuid;

        /// The redirect_uri may be optional depending on the API, but is highly recommended.
        /// This is the URL to which you want the user to be redirected after the authorization is complete.
        /// This must match the redirect URL that you have previously registered with the service.
        pub type OAuthRedirectUri = String;

        /// Include one or more scope values (space-separated) to request additional levels of access.
        /// The values will depend on the particular service.
        pub type OAuthScope = String;

        /// The state parameter serves two functions.
        /// When the user is redirected back to your app, whatever value you include as the state will also be included in the redirect.
        /// This gives your app a chance to persist data between the user being directed to the authorization server and back again,
        /// such as using the state parameter as a session key. This may be used to indicate what action in the app to perform after authorization is complete,
        /// for example, indicating which of your app’s pages to redirect to after authorization. This also serves as a CSRF protection mechanism.
        /// When the user is redirected back to your app, double check that the state value matches what you set it to originally.
        /// This will ensure an attacker can’t intercept the authorization flow.
        pub type OAuthState = String;
    }

    pub mod responses {
        use serde::{Deserialize, Serialize};

        /// User authenticated in Authmenow
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
    }
}

pub mod paths {
    use super::components;
    use actix_swagger::{Answer, ContentType};
    use actix_web::http::StatusCode;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum SessionGetResponse {
        Ok(components::responses::UserAuthenticated),
        NotAuthorized(components::responses::UserAnonymous),
    }

    impl SessionGetResponse {
        #[inline]
        pub fn answer<'a>(self) -> Answer<'a, Self> {
            let status = match self {
                Self::Ok(_) => StatusCode::OK,
                Self::NotAuthorized(_) => StatusCode::UNAUTHORIZED,
            };

            Answer::new(self)
                .status(status)
                .content_type(Some(ContentType::Json))
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum SessionCreateResponse {
        /// Session successfully created
        Ok,
    }

    impl SessionCreateResponse {
        #[inline]
        pub fn answer<'a>(self) -> Answer<'a, Self> {
            let status = match self {
                Self::Ok => StatusCode::OK,
            };

            Answer::new(self)
                .status(status)
                .content_type(Some(ContentType::Json))
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum SessionDeleteResponse {
        /// Session successfully deleted
        Ok,
    }

    impl SessionDeleteResponse {
        #[inline]
        pub fn answer<'a>(self) -> Answer<'a, Self> {
            let status = match self {
                Self::Ok => StatusCode::OK,
            };

            Answer::new(self)
                .status(status)
                .content_type(Some(ContentType::Json))
        }
    }

    pub mod register_request {
        use super::components::responses;
        use actix_swagger::{Answer, ContentType};
        use actix_web::http::StatusCode;
        use serde::Serialize;

        #[derive(Debug, Serialize)]
        #[serde(untagged)]
        pub enum Response {
            Created(responses::RegistrationRequestCreated),
            BadRequest(responses::RegisterFailed),
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
                    Self::Unexpected => None,
                    _ => Some(ContentType::Json),
                };

                Answer::new(self).status(status).content_type(content_type)
            }
        }
    }

    pub mod register_confirmation {
        use super::components::responses;
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
                    Self::Unexpected => None,
                    _ => Some(ContentType::Json),
                };

                Answer::new(self).status(status).content_type(content_type)
            }
        }
    }

    pub mod oauth_authorize_request {
        use super::components::parameters;
        use actix_swagger::Answer;
        use actix_web::http::StatusCode;
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(untagged)]
        pub enum Response {
            /// User redirected to `redirect_uri` with `error` or `code`
            ///
            /// ### Possible errors:
            /// - If the user denies the authorization request, the server will redirect the user back to the redirect URL with error=`access_denied` in the query string, and no code will be present. It is up to the app to decide what to display to the user at this point.
            /// - `invalid_request` — The request is missing a required parameter, includes an invalid parameter value, or is otherwise malformed.
            /// - `unsupported_response_type` — The authorization server does not support obtaining an authorization code using this method.
            /// - `invalid_scope` — The requested scope is invalid, unknown, or malformed.
            /// - `server_error` — The authorization server encountered an unexpected condition which prevented it from fulfilling the request.
            /// - `temporarily_unavailable` — The authorization server is currently unable to handle the request due to a temporary overloading or maintenance of the server.
            ///
            /// [OAuth2 Possible Errors](https://www.oauth.com/oauth2-servers/server-side-apps/possible-errors/)
            SeeOther,

            BadRequest,

            NotFound,

            InternalServerError,
        }

        impl Response {
            #[inline]
            pub fn answer<'a>(self) -> Answer<'a, Self> {
                let status = match self {
                    Self::SeeOther => StatusCode::SEE_OTHER,
                    Self::NotFound => StatusCode::NOT_FOUND,
                    Self::BadRequest => StatusCode::BAD_REQUEST,
                    Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
                };

                Answer::new(self).status(status)
            }
        }

        #[derive(Debug, Serialize, Deserialize)]
        pub struct QueryStruct {
            pub response_type: parameters::OAuthResponseType,
            pub client_id: parameters::OAuthClientId,
            pub redirect_uri: parameters::OAuthRedirectUri,
            pub scope: Option<parameters::OAuthScope>,
            pub state: Option<parameters::OAuthState>,
        }

        pub type Query = actix_web::web::Query<QueryStruct>;
    }
}
