use sqlx::postgres::PgPoolOptions;

type DbPool = sqlx::PgPool;

pub struct Database {
    pub(crate) pool: DbPool,
}

impl Database {
    pub fn new(connection_url: String, size: u32) -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(size)
            .connect_lazy_with(connection_url.parse().expect("Bad connection url!"));

        Self { pool }
    }
}

impl Clone for Database {
    fn clone(&self) -> Database {
        Database {
            pool: self.pool.clone(),
        }
    }
}
