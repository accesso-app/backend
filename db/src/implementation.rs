use crate::schema::*;
use accesso_core::{
    contracts::{
        AccessTokenRepo, AuthCodeRepo, ClientRepo, GetUserBySessionError, RegisterUserError,
        RequestsRepo, SaveRegisterRequestError, SessionCreateError, SessionRepo,
        UnexpectedDatabaseError, UserCredentials, UserRegisterForm, UserRepo,
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
    pub fn new(connection_url: String, size: u32) -> Result<Self, r2d2::Error> {
        let manager = ConnectionManager::<PgConnection>::new(connection_url);
        let pool = r2d2::Pool::builder().max_size(size).build(manager)?;

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

    fn get_user_by_access_token(
        &self,
        token: String,
    ) -> Result<models::User, GetUserBySessionError> {
        let conn = self.conn();

        users::table
            .inner_join(access_tokens::table)
            .select(users::all_columns)
            .filter(access_tokens::token.eq(token))
            .filter(access_tokens::expires_at.gt(chrono::Utc::now().naive_utc()))
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

    fn session_delete_token(&mut self, session_token: &str) -> Result<(), UnexpectedDatabaseError> {
        let conn = self.conn();

        diesel::delete(session_tokens::table)
            .filter(session_tokens::token.eq(session_token))
            .execute(&conn)
            .map(|_| ())
            .map_err(diesel_error_to_unexpected)
    }

    fn session_delete_by_user_id(
        &mut self,
        user_id: uuid::Uuid,
    ) -> Result<(), UnexpectedDatabaseError> {
        let conn = self.conn();

        diesel::delete(session_tokens::table)
            .filter(session_tokens::user_id.eq(user_id))
            .execute(&conn)
            .map(|_| ())
            .map_err(diesel_error_to_unexpected)
    }
}

impl ClientRepo for Database {
    fn client_find_by_id(
        &self,
        id: uuid::Uuid,
    ) -> Result<Option<models::Client>, UnexpectedDatabaseError> {
        let conn = self.conn();

        clients::table
            .filter(clients::id.eq(id))
            .get_result::<Client>(&conn)
            .map(Into::into)
            .optional()
            .map_err(diesel_error_to_unexpected)
    }
}

#[derive(Identifiable, Insertable, PartialEq, Queryable)]
struct Client {
    id: uuid::Uuid,
    redirect_uri: Vec<String>,
    secret_key: String,
    title: String,
    allowed_registrations: bool,
}

impl Into<models::Client> for Client {
    fn into(self) -> models::Client {
        models::Client {
            id: self.id,
            redirect_uri: self.redirect_uri,
            secret_key: self.secret_key,
            title: self.title,
            allowed_registrations: self.allowed_registrations,
        }
    }
}

impl AuthCodeRepo for Database {
    fn auth_code_create(
        &self,
        code: models::AuthorizationCode,
    ) -> Result<models::AuthorizationCode, UnexpectedDatabaseError> {
        let conn = self.conn();

        diesel::insert_into(authorization_codes::table)
            .values(AuthorizationCode::from(code))
            .get_result::<AuthorizationCode>(&conn)
            .map(Into::into)
            .map_err(diesel_error_to_unexpected)
    }

    fn auth_code_read(
        &self,
        code: String,
    ) -> Result<Option<models::AuthorizationCode>, UnexpectedDatabaseError> {
        let conn = self.conn();

        authorization_codes::table
            .filter(authorization_codes::code.eq(code))
            .get_result::<AuthorizationCode>(&conn)
            .map(Into::into)
            .optional()
            .map_err(diesel_error_to_unexpected)
    }
}

impl AccessTokenRepo for Database {
    fn access_token_create(
        &self,
        token: models::AccessToken,
    ) -> Result<models::AccessToken, UnexpectedDatabaseError> {
        let conn = self.conn();

        diesel::insert_into(access_tokens::table)
            .values(AccessToken::from(token))
            .get_result::<AccessToken>(&conn)
            .map(Into::into)
            .map_err(diesel_error_to_unexpected)
    }
}

#[derive(Identifiable, Insertable, PartialEq, Queryable)]
#[primary_key(code)]
struct AuthorizationCode {
    client_id: uuid::Uuid,
    code: String,
    created_at: chrono::NaiveDateTime,
    redirect_uri: String,
    scope: Option<Vec<String>>,
    user_id: uuid::Uuid,
}

impl From<models::AuthorizationCode> for AuthorizationCode {
    fn from(authorization_code: models::AuthorizationCode) -> Self {
        Self {
            client_id: authorization_code.client_id,
            code: authorization_code.code,
            created_at: authorization_code.created_at,
            redirect_uri: authorization_code.redirect_uri,
            scope: if authorization_code.scopes.is_empty() {
                None
            } else {
                Some(authorization_code.scopes)
            },
            user_id: authorization_code.user_id,
        }
    }
}

impl Into<models::AuthorizationCode> for AuthorizationCode {
    fn into(self) -> models::AuthorizationCode {
        models::AuthorizationCode {
            client_id: self.client_id,
            code: self.code,
            created_at: self.created_at,
            redirect_uri: self.redirect_uri,
            scopes: self.scope.unwrap_or(vec![]),
            user_id: self.user_id,
        }
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
    pub password_hash: String,
    pub first_name: String,
    pub last_name: String,
}

impl Into<models::User> for User {
    fn into(self) -> models::User {
        models::User {
            id: self.id,
            email: self.email,
            password_hash: self.password_hash,
            first_name: self.first_name,
            last_name: self.last_name,
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

#[derive(Identifiable, Insertable, Queryable)]
#[primary_key(token)]
pub struct AccessToken {
    pub client_id: uuid::Uuid,
    pub token: String,
    pub user_id: uuid::Uuid,
    pub scopes: Vec<String>,
    pub expires_at: chrono::NaiveDateTime,
}

impl From<models::AccessToken> for AccessToken {
    fn from(token: models::AccessToken) -> Self {
        Self {
            client_id: token.client_id,
            token: token.token,
            user_id: token.user_id,
            scopes: token.scopes,
            expires_at: token.expires_at,
        }
    }
}

impl Into<models::AccessToken> for AccessToken {
    fn into(self) -> models::AccessToken {
        models::AccessToken {
            client_id: self.client_id,
            token: self.token,
            user_id: self.user_id,
            scopes: self.scopes,
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
