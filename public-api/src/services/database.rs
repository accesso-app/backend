use authmenow_db::schema::*;
use authmenow_public_app::{
    contracts::{RequestsRepo, SaveRegisterRequestError, UnexpectedDatabaseError, UserRepo},
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
    fn has_user_with_email(&self, email: String) -> Result<bool, UnexpectedDatabaseError> {
        let conn = self.conn();

        Ok(users::table
            .filter(users::email.eq(email))
            .count()
            .get_result::<i64>(&conn)
            .map_err(|_| UnexpectedDatabaseError)?
            > 0)
    }
}

impl RequestsRepo for Database {
    fn save_register_request(
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

fn diesel_error_to_save_register_error(_: diesel::result::Error) -> SaveRegisterRequestError {
    SaveRegisterRequestError::UnexpectedError
}
