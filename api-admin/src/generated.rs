#![allow(dead_code)]
use thiserror;
use serde_json;

pub mod api {
    use actix_swagger::{Api, Method};
    use actix_web::{
        dev::{AppService, Handler, HttpServiceFactory},
        FromRequest,
    };
    use std::future::Future;

    pub struct AccessoAdminApi {
        api: Api,
    }

    pub fn create() -> AccessoAdminApi {
        AccessoAdminApi { api: Api::new() }
    }

    impl HttpServiceFactory for AccessoAdminApi {
        fn register(self, config: &mut AppService) {
            self.api.register(config);
        }
    }

    impl AccessoAdminApi {
        pub fn bind_session_get<F, T, R>(mut self, handler: F) -> Self
            where
                F: Handler<T, R>,
                T: FromRequest + 'static,
                R: Future<
                    Output = Result<
                        super::paths::session_get::Response,
                        super::paths::session_get::Error,
                    >,
                > + 'static,
        {
            self.api = self
                .api
                .bind("/api/session/get".to_owned(), Method::POST, handler);
            self
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

            let header = req
                .headers()
                .get(name)
                .ok_or_else(|| header_error.clone())?;
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
                match extract_header(req, "Authorization".to_string()) {
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

    pub mod session_get {
        use super::responses;
        use actix_swagger::ContentType;
        use actix_web::http::StatusCode;
        use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
        use serde::Serialize;

        #[derive(Debug, Serialize)]
        #[serde(untagged)]
        pub enum Response {
            Ok(responses::SessionGetSuccess),
        }

        #[derive(Debug, Serialize, thiserror::Error)]
        pub enum Error {
            #[error(transparent)]
            Unauthorized(#[serde(skip)] eyre::Report),
            #[error(transparent)]
            Unexpected(
                #[from]
                #[serde(skip)]
                eyre::Report,
            ),
        }

        impl Responder for Response {
            fn respond_to(self, _: &HttpRequest) -> HttpResponse {
                match self {
                    Response::Ok(r) => HttpResponse::build(StatusCode::OK).json(r),
                }
            }
        }

        impl ResponseError for Error {
            fn status_code(&self) -> StatusCode {
                match self {
                    Error::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
                    Error::Unauthorized(_) => StatusCode::UNAUTHORIZED,
                }
            }

            fn error_response(&self) -> HttpResponse {
                let content_type: Option<ContentType> = match self {
                    _ => None,
                };

                let mut res = &mut HttpResponse::build(self.status_code());
                if let Some(content_type) = content_type {
                    res = res.content_type(content_type.to_string());

                    match content_type {
                        ContentType::Json => res.body(serde_json::to_string(self).unwrap()),
                        ContentType::FormData => res.body(serde_plain::to_string(self).unwrap()),
                    }
                } else {
                    HttpResponse::build(self.status_code()).finish()
                }
            }
        }
    }
}
