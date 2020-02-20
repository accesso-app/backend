use crate::generated::paths::oauth_authorize_request as authorize;
use crate::models::Client;
use crate::DbPool;
use actix_swagger::Answer;
use actix_web::web;

enum AuthorizeError {
    ClientNotFound,
    UnexpectedError,
    UnknownRedirectUri,
    UnknownScope { redirect_uri: String },
}

use diesel::result::Error as DieselError;

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
) -> Answer<'static, authorize::Response> {
    let input_redirect_uri = query.redirect_uri.clone();
    let state = query.state.clone();

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
