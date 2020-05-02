use super::UnexpectedDatabaseError;
use crate::models::{Client, User};
use uuid::Uuid;

#[cfg(test)]
use mockall::*;

#[cfg_attr(test, automock)]
pub trait ClientRepo {
    fn client_find_by_id(id: Uuid) -> Result<Option<Client>, UnexpectedDatabaseError>;
}
