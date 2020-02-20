use authmenow_db::schema::clients;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct Client {
    pub id: uuid::Uuid,
    pub redirect_uri: Vec<String>,
    pub secret_key: String,
    pub scopes: Vec<String>,
    pub title: String,
}

impl Client {
    /// Get one client by its uuid
    pub fn find_by_id(
        conn: &PgConnection,
        client_id: uuid::Uuid,
    ) -> Result<Self, diesel::result::Error> {
        use authmenow_db::schema::clients::dsl::*;

        clients.filter(id.eq(client_id)).first(conn)
    }

    pub fn has_redirect_uri(&self, redirect_uri: &str) -> bool {
        self.redirect_uri
            .iter()
            .find(|uri| *uri == redirect_uri)
            .is_some()
    }

    pub fn has_scope(&self, expected_scope: &str) -> bool {
        self.scopes
            .iter()
            .find(|scope| *scope == expected_scope)
            .is_some()
    }
}
