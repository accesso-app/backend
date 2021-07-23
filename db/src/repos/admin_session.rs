use crate::Database;
use accesso_core::models::{AdminAccessToken, AdminSession};
use accesso_core::contracts::GetUserBySessionError;
use accesso_core::contracts::repo::AdminSessionRepo;
use crate::mappers::sqlx_error_to_get_user_by_session_error;
use uuid::Uuid;

#[async_trait]
impl AdminSessionRepo for Database {
    async fn get_admin_session_token(
        &self,
        token: String,
    ) -> Result<AdminSession, GetUserBySessionError> {
        sqlx::query_as!(
            AdminSession,
            // language=PostgreSQL
            r#"
            SELECT admin_session_tokens.*
                FROM admin_session_tokens
                WHERE admin_session_tokens.token = $1
            "#,
            token
        )
            .fetch_one(&self.pool)
            .await
            .map(Into::into)
            .map_err(sqlx_error_to_get_user_by_session_error)
    }

    async fn get_access_token(&self, user_id: Uuid) -> Result<AdminAccessToken, GetUserBySessionError> {
        sqlx::query_as!(
            AdminAccessToken,
            // language=PostgreSQL
            r#"
            SELECT aat.*
                FROM admins_access_tokens aat
                         INNER JOIN users ON aat.user_id = users.id
                WHERE aat.user_id = $1
                  AND aat.expires_at > $2;
            "#,
            user_id,
            chrono::Utc::now()
        )
            .fetch_one(&self.pool)
            .await
            .map(Into::into)
            .map_err(sqlx_error_to_get_user_by_session_error)

    }
}
