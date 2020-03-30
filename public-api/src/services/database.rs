use authmenow_db::schema::*;
use authmenow_public_app::{
    contracts::{
        GetUserBySessionError, RegisterUserError, RequestsRepo, SaveRegisterRequestError,
        SessionCreateError, SessionRepo, UnexpectedDatabaseError, UserCredentials,
        UserRegisterForm, UserRepo,
    },
    models,
    session::SessionCreateForm,
};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;

type Connection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct Database {
    pool: DbPool,
}

impl Database {
    pub fn new(connection_url: String) -> Result<Self, r2d2::Error> {
        let manager = ConnectionManager::<PgConnection>::new(connection_url);
        let pool = r2d2::Pool::builder().build(manager)?;

        Ok(Self { pool })
    }

    /// Waits for at most the configured connection timeout before returning an
    /// error.
    pub fn conn(&self) -> Connection {
        self.pool.get().expect("Database connection timeout")
    }
}

impl Clone for Database {
    fn clone(&self) -> Database {
        Database {
            pool: self.pool.clone(),
        }
    }
}

impl UserRepo for Database {
    fn user_has_with_email(&self, email: String) -> Result<bool, UnexpectedDatabaseError> {
        let conn = self.conn();

        Ok(users::table
            .filter(users::email.eq(email))
            .count()
            .get_result::<i64>(&conn)
            .map_err(diesel_error_to_unexpected)?
            > 0)
    }

    fn user_register(&mut self, form: UserRegisterForm) -> Result<models::User, RegisterUserError> {
        let conn = self.conn();

        let user = User {
            id: uuid::Uuid::new_v4(),
            email: form.email,
            first_name: form.first_name,
            last_name: form.last_name,
            password_hash: form.password_hash,
        };

        diesel::insert_into(users::table)
            .values(user)
            .get_result::<User>(&conn)
            .map(Into::into)
            .map_err(diesel_error_to_register_user_error)
    }

    fn user_find_by_credentials(
        &self,
        creds: UserCredentials,
    ) -> Result<Option<models::User>, UnexpectedDatabaseError> {
        users::table
            .filter(users::email.eq(creds.email))
            .filter(users::password_hash.eq(creds.password_hash))
            .get_result::<User>(&self.conn())
            .map(Into::into)
            .optional()
            .map_err(diesel_error_to_unexpected)
    }
}

impl RequestsRepo for Database {
    fn register_request_save(
        &mut self,
        request: models::RegisterRequest,
    ) -> Result<models::RegisterRequest, SaveRegisterRequestError> {
        let conn = self.conn();

        diesel::insert_into(registration_requests::table)
            .values(RegistrationRequest::from(request))
            .get_result::<RegistrationRequest>(&conn)
            .map(Into::into)
            .map_err(diesel_error_to_save_register_error)
    }

    fn register_request_get_by_code(
        &self,
        code: String,
    ) -> Result<Option<models::RegisterRequest>, UnexpectedDatabaseError> {
        let conn = self.conn();

        registration_requests::table
            .filter(registration_requests::confirmation_code.eq(code))
            .filter(registration_requests::expires_at.gt(chrono::Utc::now().naive_utc()))
            .get_result::<RegistrationRequest>(&conn)
            .map(Into::into)
            .optional()
            .map_err(diesel_error_to_unexpected)
    }

    fn register_requests_delete_all_for_email(
        &mut self,
        email: String,
    ) -> Result<usize, UnexpectedDatabaseError> {
        let conn = self.conn();

        diesel::delete(registration_requests::table)
            .filter(registration_requests::email.eq(email))
            .execute(&conn)
            .map_err(diesel_error_to_unexpected)
    }
}

impl SessionRepo for Database {
    fn get_user_by_session_token(
        &self,
        token: String,
    ) -> Result<models::User, GetUserBySessionError> {
        let conn = self.conn();

        users::table
            .inner_join(session_tokens::table)
            .select(users::all_columns)
            .filter(session_tokens::token.eq(token))
            .filter(session_tokens::expires_at.gt(chrono::Utc::now().naive_utc()))
            .first::<User>(&conn)
            .map(Into::into)
            .map_err(diesel_error_to_get_user_by_session_error)
    }

    fn session_create(
        &mut self,
        session: models::SessionToken,
    ) -> Result<models::SessionToken, SessionCreateError> {
        let conn = self.conn();

        diesel::insert_into(session_tokens::table)
            .values(SessionToken::from(session))
            .get_result::<SessionToken>(&conn)
            .map(Into::into)
            .map_err(diesel_error_to_session_create_error)
    }
}

#[derive(Identifiable, Insertable, PartialEq, Queryable)]
#[primary_key(confirmation_code)]
struct RegistrationRequest {
    confirmation_code: String,
    email: String,
    expires_at: chrono::NaiveDateTime,
}

impl From<models::RegisterRequest> for RegistrationRequest {
    fn from(model: models::RegisterRequest) -> Self {
        Self {
            confirmation_code: model.code,
            email: model.email,
            expires_at: model.expires_at,
        }
    }
}

impl Into<models::RegisterRequest> for RegistrationRequest {
    fn into(self) -> models::RegisterRequest {
        models::RegisterRequest {
            code: self.confirmation_code,
            email: self.email,
            expires_at: self.expires_at,
        }
    }
}

#[derive(Identifiable, Insertable, Queryable)]
struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub first_name: String,
    pub password_hash: String,
    pub last_name: String,
}

impl Into<models::User> for User {
    fn into(self) -> models::User {
        models::User {
            id: self.id,
            email: self.email,
            first_name: self.first_name,
            last_name: self.last_name,
            password_hash: self.password_hash,
        }
    }
}

#[derive(Identifiable, Insertable, Queryable)]
#[primary_key(token)]
pub struct SessionToken {
    pub user_id: uuid::Uuid,
    pub token: String,
    pub expires_at: chrono::NaiveDateTime,
}

impl From<models::SessionToken> for SessionToken {
    fn from(session: models::SessionToken) -> Self {
        Self {
            user_id: session.user_id,
            token: session.token,
            expires_at: session.expires_at,
        }
    }
}

impl Into<models::SessionToken> for SessionToken {
    fn into(self) -> models::SessionToken {
        models::SessionToken {
            user_id: self.user_id,
            token: self.token,
            expires_at: self.expires_at,
        }
    }
}

fn diesel_error_to_unexpected(error: diesel::result::Error) -> UnexpectedDatabaseError {
    log::error!(target: "services/database", "UNEXPECTED {:?}", error);
    UnexpectedDatabaseError
}

fn diesel_error_to_save_register_error(error: diesel::result::Error) -> SaveRegisterRequestError {
    log::error!(target: "services/database", "UNEXPECTED {:?}", error);
    SaveRegisterRequestError::Unexpected
}

fn diesel_error_to_register_user_error(err: diesel::result::Error) -> RegisterUserError {
    use diesel::result::{DatabaseErrorKind, Error as DieselError};

    match err {
        DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
            RegisterUserError::EmailAlreadyExists
        }
        error => {
            log::error!(target: "services/database", "UNEXPECTED {:?}", error);
            RegisterUserError::Unexpected
        }
    }
}

fn diesel_error_to_get_user_by_session_error(err: diesel::result::Error) -> GetUserBySessionError {
    use diesel::result::Error;

    match err {
        Error::NotFound => GetUserBySessionError::NotFound,
        error => {
            log::error!(target: "services/database", "UNEXPECTED {:?}", error);
            GetUserBySessionError::Unexpected
        }
    }
}

fn diesel_error_to_session_create_error(err: diesel::result::Error) -> SessionCreateError {
    use diesel::result::{DatabaseErrorKind, Error as DieselError};

    match err {
        DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
            SessionCreateError::TokenAlreadyExists
        }
        error => {
            log::error!(target: "services/database", "UNEXPECTED {:?}", error);
            SessionCreateError::Unexpected
        }
    }
}
