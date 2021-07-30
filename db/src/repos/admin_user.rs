use crate::entities::AdminUser;
use crate::mappers::sqlx_error_to_user_create_error;
use crate::Database;
use accesso_core::contracts::repo::{RepoResult, AdminUserCreateError, AdminUserRepo};
use accesso_core::models;
use sqlx::types::Uuid;

#[async_trait]
impl AdminUserRepo for Database {
    async fn user_find_by_id(&self, user_id: Uuid) -> RepoResult<Option<models::AdminUser>> {
        Ok(sqlx::query_as!(
            AdminUser,
            // language=PostgreSQL
            r#"
            SELECT id, accesso_id, first_name, last_name
            FROM admin_users
            WHERE admin_users.id = $1
            "#,
            user_id
        )
            .fetch_optional(&self.pool)
            .await?
            .map(Into::into))
    }

    async fn user_find_by_accesso(&self, accesso_id: Uuid) -> RepoResult<Option<models::AdminUser>> {
        Ok(sqlx::query_as!(
            AdminUser,
            // language=PostgreSQL
            r#"
            SELECT id, accesso_id, first_name, last_name
            FROM users
            WHERE users.accesso_id = $1
            "#,
            accesso_id
        )
            .fetch_optional(&self.pool)
            .await?
            .map(Into::into))
    }

    async fn user_update(&self, user: models::AdminUser) -> RepoResult<models::AdminUser> {
        let updated = sqlx::query_as!(
            AdminUser,
            // language=PostgreSQL
            r#"
            UPDATE users
            SET (accesso_id, first_name, last_name) = ($1, $2, $3)
            WHERE id = $4
            RETURNING id, accesso_id, first_name, last_name
            "#,
            user.accesso_id,
            user.first_name,
            user.last_name,
            user.id
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(updated.into())
    }

    async fn user_create(&self, user: models::AdminUserCreate) -> Result<models::AdminUser, AdminUserCreateError> {
        let user = sqlx::query_as!(
            AdminUser,
            // language=PostgreSQL
            r#"
            INSERT INTO users (accesso_id, first_name, last_name)
            VALUES ($1, $2, $3)
            RETURNING id, accesso_id, first_name, last_name
            "#,
            user.accesso_id,
            user.first_name,
            user.last_name
        )
            .fetch_one(&self.pool)
            .await
            .map_err(sqlx_error_to_user_create_error)?;

        Ok(user.into())
    }
}
