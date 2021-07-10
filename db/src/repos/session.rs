use accesso_core::contracts::repo::SessionRepo;
use accesso_core::contracts::{GetUserBySessionError, SessionCreateError, UnexpectedDatabaseError};
use accesso_core::models;

use crate::entities::{SessionToken, User};
use crate::mappers::{sqlx_error_to_get_user_by_session_error, sqlx_error_to_session_create_error};
use crate::Database;

#[async_trait]
impl SessionRepo for Database {
    async fn get_user_by_session_token(
        &self,
        token: String,
    ) -> Result<models::User, GetUserBySessionError> {
        sqlx::query_as!(
            User,
            // language=PostgreSQL
            r#"
            SELECT users.*
                FROM users
                         INNER JOIN session_tokens st ON users.id = st.user_id
                WHERE st.token = $1
                  AND st.expires_at > $2
            "#,
            token,
            chrono::Utc::now()
        )
        .fetch_one(&self.pool)
        .await
        .map(Into::into)
        .map_err(sqlx_error_to_get_user_by_session_error)
    }

    async fn get_user_by_access_token(
        &self,
        token: String,
    ) -> Result<models::User, GetUserBySessionError> {
        sqlx::query_as!(
            User,
            // language=PostgreSQL
            r#"
            SELECT users.*
                FROM users
                         INNER JOIN user_registrations ON users.id = user_registrations.user_id
                         INNER JOIN access_tokens ON user_registrations.id = access_tokens.registration_id
                WHERE access_tokens.token = $1
                  AND access_tokens.expires_at > $2;
            "#,
            token,
            chrono::Utc::now()
        )
            .fetch_one(&self.pool)
            .await
            .map(Into::into)
            .map_err(sqlx_error_to_get_user_by_session_error)
    }

    async fn session_create(
        &self,
        session: models::SessionToken,
    ) -> Result<models::SessionToken, SessionCreateError> {
        let session = SessionToken::from(session);

        sqlx::query_as!(
            SessionToken,
            // language=PostgreSQL
            r#"
            INSERT INTO session_tokens
                (user_id, token, expires_at)
                VALUES ($1, $2, $3)
                RETURNING user_id, token, expires_at
            "#,
            session.user_id,
            session.token,
            session.expires_at
        )
        .fetch_one(&self.pool)
        .await
        .map_err(sqlx_error_to_session_create_error)
        .map(Into::into)
    }

    async fn session_delete_token(
        &self,
        session_token: &str,
    ) -> Result<(), UnexpectedDatabaseError> {
        sqlx::query!(
            // language=PostgreSQL
            r#"
            DELETE
            FROM session_tokens
            WHERE token = $1
            "#,
            session_token
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn session_delete_by_user_id(
        &self,
        user_id: uuid::Uuid,
    ) -> Result<(), UnexpectedDatabaseError> {
        sqlx::query!(
            // language=PostgreSQL
            r#"
            DELETE
            FROM session_tokens
            WHERE user_id = $1
            "#,
            user_id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
