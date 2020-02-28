use crate::generated::paths::oauth_authorize_request as authorize;
use crate::models::{Client, User};
use crate::DbPool;
use actix_swagger::Answer;
use actix_web::{dev, web, FromRequest, HttpMessage};

enum AuthorizeError {
    ClientNotFound,
    UnexpectedError,
    UnknownRedirectUri,
    UnknownScope { redirect_uri: String },
}

use diesel::result::Error as DieselError;

#[derive(Debug)]
pub struct AuthPrefer {
    user: Option<User>,
}

impl AuthPrefer {
    fn from_user(user: User) -> Self {
        Self { user: Some(user) }
    }

    fn new() -> Self {
        Self { user: None }
    }
}

impl FromRequest for AuthPrefer {
    type Config = ();
    type Error = actix_web::Error;
    type Future = futures::future::Ready<Result<Self, Self::Error>>;

    #[inline]
    fn from_request(req: &actix_web::HttpRequest, _: &mut dev::Payload) -> Self::Future {
        if let Some(cookie) = req.cookie("session-token") {
            if let Some(pool) = req.app_data::<web::Data<DbPool>>() {
                let conn = pool.get().unwrap();
                let result = User::find_by_token_actual(&conn, cookie.value());

                return match result {
                    Ok(user) => futures::future::ok(AuthPrefer::from_user(user)),
                    // Do not throw an error if session is invalid
                    Err(_) => futures::future::ok(AuthPrefer::new()),
                };
            };
        }

        futures::future::ok(AuthPrefer::new())
    }
}

impl From<DieselError> for AuthorizeError {
    fn from(error: DieselError) -> AuthorizeError {
        match error {
            DieselError::NotFound => AuthorizeError::ClientNotFound,
            _ => AuthorizeError::UnexpectedError,
        }
    }
}

fn handle_authorize(
    query: authorize::Query,
    pool: web::Data<DbPool>,
) -> Result<(), AuthorizeError> {
    let conn = &pool.get().unwrap();
    let client = Client::find_by_id(conn, query.client_id)?;

    if !client.has_redirect_uri(&query.redirect_uri) {
        return Err(AuthorizeError::UnknownRedirectUri);
    }

    // Now supported only one scope per request
    if let Some(scope) = &query.scope {
        if !client.has_scope(scope) {
            return Err(AuthorizeError::UnknownScope {
                redirect_uri: query.redirect_uri.to_owned(),
            });
        }
    }

    Ok(())
}

pub async fn authorize_request(
    query: authorize::Query,
    pool: web::Data<DbPool>,
    auth: AuthPrefer,
) -> Answer<'static, authorize::Response> {
    let input_redirect_uri = query.redirect_uri.clone();
    let state = query.state.clone();

    println!("{:#?}", auth);

    match handle_authorize(query, pool) {
        Err(AuthorizeError::ClientNotFound) => authorize::Response::NotFound.answer(),
        Err(AuthorizeError::UnexpectedError) => authorize::Response::SeeOther.answer().header(
            "Location".to_owned(),
            format!(
                "{uri}?error=server_error{state}",
                uri = input_redirect_uri,
                state = state_param(state)
            ),
        ),
        Err(AuthorizeError::UnknownRedirectUri) => authorize::Response::BadRequest.answer(),
        Err(AuthorizeError::UnknownScope { redirect_uri }) => {
            authorize::Response::SeeOther.answer().header(
                "Location".to_owned(),
                format!(
                    "{uri}?error=invalid_scope{state}",
                    uri = redirect_uri,
                    state = state_param(state)
                ),
            )
        }

        Ok(_) => authorize::Response::SeeOther.answer().header(
            "Location".to_owned(),
            format!(
                "{uri}?code=TEMPORARY{state}",
                uri = input_redirect_uri,
                state = state_param(state)
            ),
        ),
    }
}

fn state_param(state: Option<String>) -> String {
    match state {
        Some(state) => format!("&state={}", state),
        None => String::new(),
    }
}
