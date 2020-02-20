use crate::generated::paths::oauth_authorize_request as authreq;
use crate::models::Client;
use crate::DbPool;
use actix_swagger::Answer;
use actix_web::web;

enum AuthorizeError {
    ClientNotFound,
    UnexpectedError,
}

impl From<diesel::result::Error> for AuthorizeError {
    fn from(error: diesel::result::Error) -> AuthorizeError {
        use diesel::result::Error;
        match error {
            Error::NotFound => AuthorizeError::ClientNotFound,
            _ => AuthorizeError::UnexpectedError,
        }
    }
}

fn handle_authorize(client_id: uuid::Uuid, pool: web::Data<DbPool>) -> Result<(), AuthorizeError> {
    let conn = &pool.get().unwrap();
    let client = Client::find_by_id(conn, client_id)?;

    Ok(())
}

pub async fn authorize_request(
    query: authreq::Query,
    pool: web::Data<DbPool>,
) -> Answer<'static, authreq::Response> {
    match handle_authorize(query.client_id, pool) {
        Err(AuthorizeError::ClientNotFound) => authreq::Response::NotFound.answer(),
        Err(AuthorizeError::UnexpectedError) => authreq::Response::InternalServerError.answer(),
        Ok(_) => authreq::Response::SeeOther
            .answer()
            .header("Location".to_owned(), "https://google.com"),
    }
}
