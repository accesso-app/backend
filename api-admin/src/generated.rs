#![allow(dead_code)]
#![allow(clippy::from_over_into)]
#![allow(clippy::enum_variant_names)]

pub mod api {
    use actix_swagger::{Api, Method};
    use actix_web::{
        dev::{AppService, Handler, HttpServiceFactory},
        FromRequest, Responder,
    };
    use std::future::Future;

    pub struct AccessoAdminAPIInternal {
        api: Api,
    }

    pub fn create() -> AccessoAdminAPIInternal {
        AccessoAdminAPIInternal { api: Api::new() }
    }

    impl HttpServiceFactory for AccessoAdminAPIInternal {
        fn register(self, config: &mut AppService) {
            self.api.register(config)
        }
    }

    impl AccessoAdminAPIInternal {
        pub fn bind_auth_params<F, T, R>(mut self, handler: F) -> Self
        where
            F: Handler<T, R>,
            T: FromRequest + 'static,
            R: Future<
                    Output = Result<
                        super::paths::auth_params::Response,
                        super::paths::auth_params::Error,
                    >,
                > + 'static,
        {
            self.api = self
                .api
                .bind("api/accesso/auth.params".into(), Method::POST, handler);
            self
        }

        pub fn bind_auth_done<F, T, R, Res>(mut self, handler: F) -> Self
        where
            F: Handler<T, R>,
            T: FromRequest + 'static,
            Res: Responder + 'static,
            R: Future<Output = Result<Res, super::paths::auth_done::Error>> + 'static,
        {
            self.api = self
                .api
                .bind("api/accesso/auth.done".into(), Method::POST, handler);
            self
        }

        pub fn bind_session_get<F, T, R, Res>(mut self, handler: F) -> Self
        where
            F: Handler<T, R>,
            T: FromRequest + 'static,
            Res: Responder + 'static,
            R: Future<Output = Result<Res, super::paths::session_get::Error>> + 'static,
        {
            self.api = self
                .api
                .bind("api/session/get".into(), Method::POST, handler);
            self
        }

    }
}

pub mod components {
    pub mod responses {
        use super::schemas;
        use serde::Serialize;
        use uuid::Uuid;

        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct AuthUrlSuccess {
            /// Accesso URL
            pub accesso_url: String,
        }

        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct AuthDoneSuccess {
            pub user_info: schemas::UserInfo,
        }

        #[derive(Debug, Serialize, thiserror::Error)]
        #[serde(rename_all = "camelCase")]
        #[error(transparent)]
        pub struct AuthDoneFailed {
            #[from]
            error: AuthDoneError,
        }

        #[derive(Debug, Serialize, thiserror::Error)]
        #[serde(rename_all = "snake_case")]
        pub enum AuthDoneError {
            #[error("Accesso failed")]
            AccessoFailed,
            #[error("Try later")]
            TryLater,
        }

        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct SessionGetSuccess {
            pub user_info: schemas::UserInfo,
        }

        #[derive(Debug, Serialize, thiserror::Error)]
        #[serde(rename_all = "camelCase")]
        #[error(transparent)]
        pub struct SessionGetFailed {
            #[from]
            error: SessionGetError,
        }

        #[derive(Debug, Serialize, thiserror::Error)]
        #[serde(rename_all = "snake_case")]
        pub enum SessionGetError {
            #[error("Accesso failed")]
            AccessoFailed,
            #[error("Try later")]
            TryLater,
        }

    }

    pub mod request_bodies {
        use serde::Deserialize;
        use uuid::Uuid;

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct AuthUrlRequestBody {
            /// oauth state
            pub state: String,
        }

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct AuthDoneRequestBody {
            /// Authorization code
            pub authorization_code: String,
        }

    }

    pub mod schemas {
        use chrono::{DateTime, Utc};
        use serde::Serialize;
        use uuid::Uuid;

        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct UserInfo {
            pub first_name: String,
            pub last_name: String,
        }

        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct User {
            pub id: Uuid,
            //pub username: String,
            pub first_name: String,
            pub last_name: String,
        }

    }
}
pub mod paths {
    use super::components::responses;

    pub mod auth_done {
        use super::responses;
        use actix_swagger::ContentType;
        use actix_web::http::StatusCode;
        use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
        use serde::Serialize;

        #[derive(Debug, Serialize, thiserror::Error)]
        #[serde(untagged)]
        pub enum Error {
            #[error(transparent)]
            BadRequest(#[from] responses::AuthDoneFailed),
            #[error("Unauthorized")]
            Unauthorized,
            #[error(transparent)]
            Unexpected(
                #[from]
                #[serde(skip)]
                eyre::Report,
            ),
        }

        #[derive(Debug, Serialize)]
        #[serde(untagged)]
        pub enum Response {
            Ok(responses::AuthDoneSuccess),
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
                    Error::BadRequest(_) => StatusCode::BAD_REQUEST,
                    Error::Unauthorized => StatusCode::UNAUTHORIZED,
                }
            }

            fn error_response(&self) -> HttpResponse {
                let content_type = match self {
                    Self::BadRequest(_) => Some(ContentType::Json),
                    _ => None,
                };

                let mut res = &mut HttpResponse::build(self.status_code());
                if let Some(content_type) = content_type {
                    res = res.content_type(content_type.to_string());

                    match content_type {
                        ContentType::Json => res.json(self),
                        ContentType::FormData => res.body(serde_plain::to_string(self).unwrap()),
                    }
                } else {
                    HttpResponse::build(self.status_code()).finish()
                }
            }
        }
    }

    pub mod auth_params {
        use super::responses;
        use actix_web::http::StatusCode;
        use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
        use serde::Serialize;

        #[derive(Debug, Serialize, thiserror::Error)]
        #[serde(untagged)]
        pub enum Error {
            #[error(transparent)]
            InternalServerError(
                #[from]
                #[serde(skip)]
                eyre::Report,
            ),
        }

        #[derive(Debug, Serialize)]
        #[serde(untagged)]
        pub enum Response {
            Ok(responses::AuthUrlSuccess),
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
                    Error::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                }
            }

            fn error_response(&self) -> HttpResponse {
                HttpResponse::build(self.status_code()).finish()
            }
        }
    }

    pub mod session_get {
        use super::responses;
        use actix_swagger::ContentType;
        use actix_web::http::StatusCode;
        use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
        use serde::Serialize;

        #[derive(Debug, Serialize, thiserror::Error)]
        #[serde(untagged)]
        pub enum Error {
            #[error(transparent)]
            BadRequest(#[from] responses::AuthDoneFailed),
            #[error("Unauthorized")]
            Unauthorized,
            #[error(transparent)]
            Unexpected(
                #[from]
                #[serde(skip)]
                eyre::Report,
            ),
        }

        #[derive(Debug, Serialize)]
        #[serde(untagged)]
        pub enum Response {
            Ok(responses::SessionGetSuccess),
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
                    Error::BadRequest(_) => StatusCode::BAD_REQUEST,
                    Error::Unauthorized => StatusCode::UNAUTHORIZED,
                }
            }

            fn error_response(&self) -> HttpResponse {
                let content_type = match self {
                    Self::BadRequest(_) => Some(ContentType::Json),
                    _ => None,
                };

                let mut res = &mut HttpResponse::build(self.status_code());
                if let Some(content_type) = content_type {
                    res = res.content_type(content_type.to_string());

                    match content_type {
                        ContentType::Json => res.json(self),
                        ContentType::FormData => res.body(serde_plain::to_string(self).unwrap()),
                    }
                } else {
                    HttpResponse::build(self.status_code()).finish()
                }
            }
        }
    }

}
