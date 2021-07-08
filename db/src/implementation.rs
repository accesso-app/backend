use sqlx::postgres::PgPoolOptions;

type DbPool = sqlx::PgPool;

pub struct Database {
    pub(crate) pool: DbPool,
}

impl Database {
    pub async fn new(connection_url: String, size: u32) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(size)
            .connect(&connection_url)
            .await?;

        Ok(Self { pool })
    }
}

impl Clone for Database {
    fn clone(&self) -> Database {
        Database {
            pool: self.pool.clone(),
        }
    }
}
