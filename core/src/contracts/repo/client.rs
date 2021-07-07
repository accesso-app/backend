use super::UnexpectedDatabaseError;
use crate::models::{AccessToken, AuthorizationCode, Client};
use uuid::Uuid;

#[cfg(feature = "testing")]
use mockall::*;

#[cfg_attr(feature = "testing", automock)]
pub trait ClientRepo {
    fn client_find_by_id(&self, id: Uuid) -> Result<Option<Client>, UnexpectedDatabaseError>;
}

#[cfg_attr(feature = "testing", automock)]
pub trait AuthCodeRepo {
    fn auth_code_create(
        &self,
        code: AuthorizationCode,
    ) -> Result<AuthorizationCode, UnexpectedDatabaseError>;

    fn auth_code_read(
        &self,
        code: String,
    ) -> Result<Option<AuthorizationCode>, UnexpectedDatabaseError>;
}

#[cfg_attr(feature = "testing", automock)]
pub trait AccessTokenRepo {
    fn access_token_create(
        &self,
        token: AccessToken,
    ) -> Result<AccessToken, UnexpectedDatabaseError>;
}
