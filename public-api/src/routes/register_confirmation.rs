use crate::generated::components::request_bodies;
use crate::generated::components::responses::{
    RegisterConfirmationFailed as ConfirmFailed, RegisterConfirmationFailedError as ErrorCode,
};
use crate::generated::paths::register_confirmation as confirm;
use crate::models::{RegistrationRequest, User};
use crate::DbPool;
use actix_swagger::Answer;
use actix_web::web;
use diesel::result::{DatabaseErrorKind, Error as DieselError};

enum HandleError {
    CodeNotFound,
    AlreadyActivated,
    Unexpected,
}

fn handle(
    body: request_bodies::RegisterConfirmation,
    pool: web::Data<DbPool>,
) -> Result<(), HandleError> {
    let conn = &pool.get().unwrap();

    match RegistrationRequest::find_by_code_actual(&conn, &body.confirmation_code) {
        Err(DieselError::NotFound) => Err(HandleError::CodeNotFound),
        Err(error) => {
            log::trace!(
                "Failed to find registration request by code {}: {:?}",
                body.confirmation_code,
                error
            );
            Err(HandleError::Unexpected)
        }
        Ok(request) => {
            let user = User::new()
                .email_set(&request.email)
                .password_set(&body.password);

            match user.create(&conn) {
                Err(DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
                    Err(HandleError::AlreadyActivated)
                }
                Err(error) => {
                    log::trace!("Failed to create user {:?}: {:?}", user, error);
                    Err(HandleError::Unexpected)
                }
                Ok(user) => {
                    log::warn!(
                        "Email 'register complete' do not sent to {} because not implemented",
                        user.email
                    );
                    let _ = RegistrationRequest::delete_all_for_email(&conn, &request.email);
                    Ok(())
                }
            }
        }
    }
}

pub async fn route(
    body: web::Json<request_bodies::RegisterConfirmation>,
    pool: web::Data<DbPool>,
) -> Answer<'static, confirm::Response> {
    match handle(body.0, pool) {
        Ok(()) => confirm::Response::Created,
        Err(HandleError::CodeNotFound) => confirm::Response::BadRequest(ConfirmFailed {
            error: ErrorCode::CodeInvalidOrExpired,
        }),
        Err(HandleError::AlreadyActivated) => confirm::Response::BadRequest(ConfirmFailed {
            error: ErrorCode::EmailAlreadyActivated,
        }),
        Err(HandleError::Unexpected) => confirm::Response::Unexpected,
    }
    .answer()
}
