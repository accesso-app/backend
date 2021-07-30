use accesso_core::models;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, sqlx::Type)]
pub(crate) struct AdminUser {
    pub(crate) id: Uuid,
    pub(crate) accesso_id: Uuid,
    pub(crate) first_name: String,
    pub(crate) last_name: String,
}

impl From<AdminUser> for models::AdminUser {
    #[inline]
    fn from(u: AdminUser) -> Self {
        Self {
            id: u.id,
            accesso_id: u.accesso_id,
            first_name: u.first_name,
            last_name: u.last_name,
        }
    }
}
