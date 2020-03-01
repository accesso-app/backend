use crate::generated::components::{request_bodies, responses};
use crate::generated::paths::register_request;
use crate::models::{RegistrationRequest, User};
use crate::DbPool;
use actix_swagger::Answer;
use actix_web::web;
use diesel::result::{DatabaseErrorKind, Error};

enum RegisterRequestError {
    EmailAlreadyRegistered,
    UnexpectedError,
}

fn handle(
    body: request_bodies::Register,
    pool: web::Data<DbPool>,
) -> Result<RegistrationRequest, RegisterRequestError> {
    let conn = &pool.get().unwrap();
    let is_busy = User::has_with_email(&conn, &body.email);

    if is_busy {
        Err(RegisterRequestError::EmailAlreadyRegistered)
    } else {
        let request = RegistrationRequest::new(&body.email);

        log::warn!("Email confirmation not sent. Because not implemented");

        match request.create(&conn) {
            Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
                Err(RegisterRequestError::EmailAlreadyRegistered)
            }
            Err(_) => Err(RegisterRequestError::UnexpectedError),
            Ok(value) => Ok(value),
        }
    }
}

pub async fn route(
    body: web::Json<request_bodies::Register>,
    pool: web::Data<DbPool>,
) -> Answer<'static, register_request::Response> {
    match handle(body.0, pool) {
        Ok(request) => {
            log::trace!(
                "Registered request: {email} — {code}",
                email = request.email,
                code = request.confirmation_code
            );
            register_request::Response::Created(responses::RegistrationRequestCreated {
                expires_at: request.expires_at.timestamp_millis(),
            })
            .answer()
        }
        Err(RegisterRequestError::EmailAlreadyRegistered) => {
            register_request::Response::BadRequest(responses::RegisterFailed {
                error: responses::RegisterFailedError::EmailAlreadyRegistered,
            })
            .answer()
        }
        Err(RegisterRequestError::UnexpectedError) => {
            register_request::Response::Unexpected.answer()
        }
    }
}
