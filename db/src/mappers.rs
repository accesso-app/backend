use sqlx::postgres::PgDatabaseError;

use accesso_core::contracts::{
    GetUserBySessionError, RegisterUserError, SaveRegisterRequestError, SessionCreateError,
    UserEditError, UserRegistrationCreateError,
};

use crate::sql_state::SqlState;

pub fn sqlx_error_to_save_register_request_error(err: sqlx::Error) -> SaveRegisterRequestError {
    use sqlx::error::Error as SqlxError;

    if let SqlxError::Database(ref e) = err {
        let pg_err = e.downcast_ref::<PgDatabaseError>();
        if pg_err.code() == SqlState::UNIQUE_VIOLATION.code() {
            return SaveRegisterRequestError::CodeAlreadyExists;
        }
    }

    println!("{:#?}", err);
    SaveRegisterRequestError::Unexpected(err.into())
}

pub fn sqlx_error_to_register_user_error(err: sqlx::Error) -> RegisterUserError {
    use sqlx::error::Error as SqlxError;

    if let SqlxError::Database(ref e) = err {
        let pg_err = e.downcast_ref::<PgDatabaseError>();
        if pg_err.code() == SqlState::UNIQUE_VIOLATION.code() {
            return RegisterUserError::EmailAlreadyExists;
        }
    }

    println!("{:#?}", err);
    RegisterUserError::Unexpected(err.into())
}

pub fn sqlx_error_to_account_edit_error(err: sqlx::Error) -> UserEditError {
    println!("{:#?}", err);
    UserEditError::Unexpected(err.into())
}

pub fn sqlx_error_to_session_create_error(err: sqlx::Error) -> SessionCreateError {
    use sqlx::Error as SqlxError;

    if let SqlxError::Database(ref e) = err {
        let pg_err = e.downcast_ref::<PgDatabaseError>();
        if pg_err.code() == SqlState::UNIQUE_VIOLATION.code() {
            return SessionCreateError::TokenAlreadyExists;
        }
    }

    println!("{:#?}", err);
    SessionCreateError::Unexpected(err.into())
}

pub fn sqlx_error_to_user_registration_error(error: sqlx::Error) -> UserRegistrationCreateError {
    use sqlx::Error as SqlxError;

    if let SqlxError::Database(ref e) = error {
        let pg_err = e.downcast_ref::<PgDatabaseError>();
        if pg_err.code() == SqlState::INVALID_FOREIGN_KEY.code() {
            if let Some("client_id") = pg_err.column() {
                return UserRegistrationCreateError::ClientDoesNotExist;
            }
            if let Some("user_id") = pg_err.column() {
                return UserRegistrationCreateError::UserDoesNotExist;
            }
        }
    }

    println!("{:#?}", error);
    UserRegistrationCreateError::Unexpected(error.into())
}

pub fn sqlx_error_to_get_user_by_session_error(err: sqlx::Error) -> GetUserBySessionError {
    use sqlx::Error;

    match err {
        Error::RowNotFound => GetUserBySessionError::NotFound,
        _ => GetUserBySessionError::Unexpected(err.into()),
    }
}
