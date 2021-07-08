use crate::{App, Service};
use accesso_core::app::session::{
    RepoError, Session, SessionCreateError, SessionCreateForm, SessionDeleteError,
    SessionDeleteStrategy, SessionResolveError,
};
use accesso_core::contracts::{
    GetUserBySessionError, Repository, SecureGenerator, UserCredentials,
};
use accesso_core::models::{SessionToken, User};
use async_trait::async_trait;

use accesso_db::chrono;
use validator::Validate;

const MAX_TOKEN_CREATE_ATTEMPTS: u8 = 10;
const SESSION_TOKEN_LIVE_DAYS: u8 = 14;

#[async_trait]
impl Session for App {
    async fn session_resolve_by_cookie(
        &self,
        cookie: String,
    ) -> Result<Option<User>, SessionResolveError> {
        let db = self.get::<Service<dyn Repository>>().unwrap();

        match db.get_user_by_session_token(cookie).await {
            Err(GetUserBySessionError::Unexpected) => Err(SessionResolveError::Unexpected),
            Err(GetUserBySessionError::NotFound) => Ok(None),
            Ok(user) => Ok(Some(user)),
        }
    }

    async fn session_resolve_by_access_token(
        &self,
        access_token: String,
    ) -> Result<Option<User>, SessionResolveError> {
        let db = self.get::<Service<dyn Repository>>().unwrap();

        match db.get_user_by_access_token(access_token).await {
            Err(GetUserBySessionError::Unexpected) => Err(SessionResolveError::Unexpected),
            Err(GetUserBySessionError::NotFound) => Ok(None),
            Ok(user) => Ok(Some(user)),
        }
    }

    async fn session_create(
        &self,
        form: SessionCreateForm,
    ) -> Result<(SessionToken, User), SessionCreateError> {
        let db = self.get::<Service<dyn Repository>>().unwrap();
        let generator = self.get::<Service<dyn SecureGenerator>>().unwrap();
        form.validate()?;

        let hashed_input_password = generator.password_hash(form.password.clone());

        let found_user = db
            .user_find_by_credentials(UserCredentials {
                email: form.email,
                password_hash: hashed_input_password.0,
            })
            .await?;

        if let Some(user) = found_user {
            if !generator.verify_hash(user.password_hash.as_bytes(), &form.password) {
                return Err(SessionCreateError::InvalidCredentials);
            }

            let mut insert_attempt = 0u8;

            let session: SessionToken = loop {
                insert_attempt += 1;

                let token = generator.generate_token();
                let result = db
                    .session_create(SessionToken {
                        user_id: user.id,
                        token,
                        expires_at: chrono::Utc::now()
                            + chrono::Duration::days(SESSION_TOKEN_LIVE_DAYS as i64),
                    })
                    .await;

                if let Err(RepoError::TokenAlreadyExists) = result {
                    if insert_attempt <= MAX_TOKEN_CREATE_ATTEMPTS {
                        continue;
                    }
                }

                break result;
            }?;

            Ok((session, user))
        } else {
            Err(SessionCreateError::InvalidCredentials)
        }
    }

    async fn session_delete(
        &self,
        user: &User,
        strategy: SessionDeleteStrategy,
    ) -> Result<(), SessionDeleteError> {
        let db = self.get::<Service<dyn Repository>>().unwrap();
        match strategy {
            SessionDeleteStrategy::All => db
                .session_delete_by_user_id(user.id)
                .await
                .map_err(|_unexpected| SessionDeleteError::Unexpected),

            SessionDeleteStrategy::Single(token) => db
                .session_delete_token(token.as_ref())
                .await
                .map_err(|_unexpected| SessionDeleteError::Unexpected),
        }
    }
}
