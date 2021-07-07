use sqlx::postgres::PgDatabaseError;

use accesso_core::contracts::{
    GetUserBySessionError, RegisterUserError, SaveRegisterRequestError, SessionCreateError,
    UnexpectedDatabaseError,
};

use crate::sql_state::SqlState;

pub fn sqlx_error_to_save_register_error(error: sqlx::Error) -> SaveRegisterRequestError {
    log::error!(target: "services/database", "UNEXPECTED {:?}", error);
    SaveRegisterRequestError::Unexpected
}

pub fn sqlx_error_to_register_user_error(err: sqlx::Error) -> RegisterUserError {
    use sqlx::error::Error as SqlxError;

    if let SqlxError::Database(ref e) = err {
        let pg_err = e.downcast_ref::<PgDatabaseError>();
        if pg_err.code() == SqlState::UNIQUE_VIOLATION.code() {
            return RegisterUserError::EmailAlreadyExists;
        }
    }

    log::error!(target: "services/database", "UNEXPECTED {:?}", err);
    RegisterUserError::Unexpected
}

pub fn sqlx_error_to_session_create_error(err: sqlx::Error) -> SessionCreateError {
    use sqlx::Error as SqlxError;

    if let SqlxError::Database(ref e) = err {
        let pg_err = e.downcast_ref::<PgDatabaseError>();
        if pg_err.code() == SqlState::UNIQUE_VIOLATION.code() {
            return SessionCreateError::TokenAlreadyExists;
        }
    }

    log::error!(target: "services/database", "UNEXPECTED {:?}", err);
    SessionCreateError::Unexpected
}

pub fn sqlx_error_to_get_user_by_session_error(err: sqlx::Error) -> GetUserBySessionError {
    use sqlx::Error;

    match err {
        Error::RowNotFound => GetUserBySessionError::NotFound,
        error => {
            log::error!(target: "services/database", "UNEXPECTED {:?}", error);
            GetUserBySessionError::Unexpected
        }
    }
}

pub fn sqlx_error_to_unexpected(error: sqlx::Error) -> UnexpectedDatabaseError {
    log::error!(target: "services/database", "UNEXPECTED {:?}", error);
    UnexpectedDatabaseError
}
