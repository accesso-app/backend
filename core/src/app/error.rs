use actix_web::http::StatusCode;
use actix_web::ResponseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AdminSessionTokenExtractorError {
    #[error("No session token present in cookie")]
    NoSessionToken,
}

impl ResponseError for AdminSessionTokenExtractorError {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }
}
