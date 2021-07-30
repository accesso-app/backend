#[cfg(feature = "testing")]
use mockall::*;

use super::RepoResult;
use crate::models::{AdminSessionToken};
use uuid::Uuid;

#[cfg_attr(feature = "testing", automock)]
#[async_trait]
pub trait AdminSessionTokenRepo {
    async fn admin_token_delete_by_user(&self, user_id: Uuid) -> RepoResult<u64>;
    async fn admin_token_delete(&self, token: String) -> RepoResult<u64>;
    async fn admin_token_find(&self, token: String) -> RepoResult<Option<AdminSessionToken>>;
    // QUES: maybe use user: models::User instead of Uuid, because type of id is a detail of implementation
    async fn admin_token_find_by_user(&self, user_id: Uuid) -> RepoResult<Option<AdminSessionToken>>;
    async fn admin_token_create(&self, token: AdminSessionToken) -> RepoResult<AdminSessionToken>;
}

#[cfg(feature = "testing")]
#[async_trait]
impl AdminSessionTokenRepo for crate::contracts::MockDb {
    async fn admin_token_delete_by_user(&self, user_id: Uuid) -> RepoResult<u64> {
        self.session_tokens.token_delete_by_user(user_id).await
    }

    async fn admin_token_delete(&self, token: String) -> RepoResult<u64> {
        self.session_tokens.token_delete(token).await
    }

    async fn admin_token_find(&self, token: String) -> RepoResult<Option<AdminSessionToken>> {
        self.session_tokens.token_find(token).await
    }

    async fn admin_token_find_by_user(&self, user_id: Uuid) -> RepoResult<Option<AdminSessionToken>> {
        self.session_tokens.token_find_by_user(user_id).await
    }

    async fn admin_token_create(&self, token: AdminSessionToken) -> RepoResult<AdminSessionToken> {
        self.session_tokens.token_create(token).await
    }
}
