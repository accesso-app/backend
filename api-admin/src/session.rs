use actix_web::{error, web, HttpResponse, ResponseError, HttpResponseBuilder};
use actix_web::{error::{ErrorInternalServerError, ErrorUnauthorized}};
use futures::Future;
use std::pin::Pin;
use accesso_core::app::oauth::exchange::OAuthExchange;
use accesso_core::app::admin_session::{AdminSession, RepoError};
use crate::generated::components::parameters::AccessToken;
use actix_web::http::{StatusCode, header};
use actix_web::web::Data;
use accesso_core::models::{User, SessionToken};
use accesso_db::chrono;
use uuid::Uuid;
use derive_more::{Display, Error};
use accesso_core::contracts::{SomeError, SecureGenerator, Repository};
use accesso_core::app::session::{SessionCreateForm, SessionCreateError};
use accesso_app::{Service, App};
use std::fmt::{Display, Formatter};


const MAX_TOKEN_CREATE_ATTEMPTS: u8 = 10;
const SESSION_TOKEN_LIVE_DAYS: u8 = 14;

#[derive(Debug)]
pub struct Session {
    pub user: accesso_core::models::User,
    pub token: String,
}

impl actix_web::FromRequest for Session {
    type Config = ();
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self, Self::Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        use accesso_core::app::session::{Session, SessionResolveError::Unexpected};

        let req = req.clone();

        Box::pin(async move {
            let req = req.clone();
            let session_config = req.app_data::<web::Data<accesso_app::SessionCookieConfig>>();
            let app = req.app_data::<web::Data<accesso_app::App>>();

            let session_config = session_config.ok_or(SessionGetError::InternalError)?;
            let app = app.ok_or(SessionGetError::InternalError)?;

            let ref cookie = req.clone().cookie(&session_config.name).ok_or(SessionGetError::Unauthorized)?;
                // tracing::warn!("No cookie found!");

            let session_token = cookie.value().to_owned();
            // We also take expired sessions in order to find user
            let session_result: Result<accesso_core::models::AdminSession, Box<dyn std::error::Error>> = match app.admin_session_resolve_by_cookie(session_token.clone()).await {
                Err(Unexpected(_)) => return Err(ErrorInternalServerError(Null)),
                Ok(None) => return Err(ErrorInternalServerError(Null)),
                Ok(session) => Ok(session.unwrap()),
            };

            let session = session_result.unwrap();
            let is_expired_session = session.expires_at < chrono::Utc::now();
            if !is_expired_session {
                let user = app.user_get_by_id(session.user_id).await;
                return match user {
                    Ok(Some(user)) => Ok(Self {
                        user,
                        token: session_token.clone()
                    }),
                    Ok(None) => Err(ErrorUnauthorized(Null)),
                    Err(_) => Err(ErrorInternalServerError(Null)),
                }
            }

            let user = resolveUserByToken(app, session.user_id).await;
            let create_result =  match user {
                Ok(user) => session_create(app, user).await,
                Err(_) => return Err(ErrorUnauthorized(Null)),
            };

            if let (session, user) = create_result.unwrap() {
               Ok( Self {
                    user,
                    token: session.token
                })
            } else {
                Err(ErrorInternalServerError(Null))
            }
        })
    }
}

#[derive(Debug, Display, Error)]
enum SessionGetError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "unauthorized")]
    Unauthorized,

    #[display(fmt = "timeout")]
    Timeout,
}

impl ResponseError for SessionGetError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            SessionGetError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            SessionGetError::Unauthorized => StatusCode::UNAUTHORIZED,
            SessionGetError::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}

async fn session_create(app: &Data<App>, user: User) -> Result<(SessionToken, User), SessionCreateError> {
    let db = app.get::<Service<dyn Repository>>()?;
    let generator = app.get::<Service<dyn SecureGenerator>>()?;
    let mut insert_attempt = 0u8;

    let session: SessionToken = loop {
        insert_attempt += 1;

        let token = generator.generate_token();
        let result = db
            .session_create(SessionToken {
                user_id: user.id,
                token,
                expires_at: chrono::Utc::now()
                    + chrono::Duration::days(SESSION_TOKEN_LIVE_DAYS as i64),
            })
            .await;

        if let Err(RepoError::TokenAlreadyExists) = result {
            if insert_attempt <= MAX_TOKEN_CREATE_ATTEMPTS {
                continue;
            }
        }

        break result;
    }?;

    Ok((session, user))
}

async fn resolveUserByToken(app: &Data<accesso_app::App>, user_id: Uuid) -> Result<User, TokenResolveError> {
    let token = app.get_admin_access_token(user_id).await;
    if token.is_err() {
        return Err(TokenResolveError::AccessTokenNotFound);
    }

    let mut client = awc::Client::default();

    let mut user_response = client.get("http://localhost:9010/viewer")
        .insert_header(("User-Agent", "Actix-web"))
        .insert_header(("X-Access-Token", token.unwrap().unwrap().token))
        .send()
        .await
        .unwrap();

    println!("User response: {:?}", user_response);

    match user_response.status() {
        StatusCode::OK => {},
        _ => return Err(TokenResolveError::UserNotFound),
    }

    let user: User = user_response.json().await.unwrap();

    Ok(user)

}

#[derive(Debug, thiserror::Error)]
pub enum TokenResolveError {
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
    #[error("Access token not found")]
    AccessTokenNotFound,
    #[error("Viewer get error")]
    UserNotFound,
}

#[derive(Debug, serde::Serialize)]
struct Null;

impl std::fmt::Display for Null {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
