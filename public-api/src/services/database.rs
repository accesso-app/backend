use authmenow_db::schema::*;
use authmenow_public_app::{
    contracts::{
        RegisterUserError, RequestsRepo, SaveRegisterRequestError, UnexpectedDatabaseError,
        UserRegisterForm, UserRepo,
    },
    models,
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

    fn register_user(
        &self,
        form: UserRegisterForm,
    ) -> Result<models::CreatedUser, RegisterUserError> {
        let conn = self.conn();

        let user = NewUser {
            id: uuid::Uuid::new_v4(),
            email: form.email,
            first_name: form.first_name,
            last_name: form.last_name,
            password_hash: form.password_hash,
        };

        diesel::insert_into(users::table)
            .values(user)
            .get_result::<NewUser>(&conn)
            .map(Into::into)
            .map_err(diesel_error_to_register_user_error)
    }
}

impl RequestsRepo for Database {
    fn register_request_save(
        &self,
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
        &self,
        email: String,
    ) -> Result<usize, UnexpectedDatabaseError> {
        let conn = self.conn();

        diesel::delete(registration_requests::table)
            .filter(registration_requests::email.eq(email))
            .execute(&conn)
            .map_err(diesel_error_to_unexpected)
    }
}

#[derive(Identifiable, Insertable, PartialEq, Queryable)]
#[primary_key(confirmation_code)]
struct RegistrationRequest {
    confirmation_code: String,
    email: String,
    expires_at: chrono::NaiveDateTime,
}

#[derive(Identifiable, Insertable, Queryable)]
#[table_name = "users"]
pub struct NewUser {
    pub id: uuid::Uuid,
    pub email: String,
    pub first_name: String,
    pub password_hash: String,
    pub last_name: String,
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

impl Into<models::CreatedUser> for NewUser {
    fn into(self) -> models::CreatedUser {
        models::CreatedUser {
            id: self.id,
            email: self.email,
            first_name: self.first_name,
            last_name: self.last_name,
            password_hash: self.password_hash,
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
