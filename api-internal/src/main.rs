use std::env;
use std::net::Ipv4Addr;

use actix_web::http::StatusCode;
use actix_web::web::Json;
use actix_web::{self, web, App, Error, HttpResponse, HttpServer, ResponseError};
use lambda_web::{is_running_on_lambda, run_actix_on_lambda, LambdaError};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct Pet {
    name: String,
    id: Uuid,
    #[serde(with = "time::serde::iso8601")]
    created_at: OffsetDateTime,
}

#[derive(Deserialize)]
struct PetId {
    pub id: Uuid,
}

async fn pets_get(path: web::Path<PetId>) -> Result<Json<Pet>, Error> {
    Ok(Json(Pet {
        name: "Dog".into(),
        id: path.id,
        created_at: OffsetDateTime::now_utc(),
    }))
}

use sqlx::FromRow;

#[derive(Debug, thiserror::Error)]
pub enum UnexpectedDatabaseError {
    #[error("Unexpected database error: {0}")]
    SqlxError(#[from] sqlx::error::Error),
}

#[derive(Serialize)]
struct JsonError {
    error: String,
}

impl ResponseError for UnexpectedDatabaseError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            UnexpectedDatabaseError::SqlxError(e) => {
                println!("{:#?}", e)
            }
        }
        HttpResponse::build(self.status_code()).json(JsonError {
            error: "internal_server_error".to_string(),
        })
    }
}

#[derive(Debug, FromRow, Serialize)]
pub(crate) struct Client {
    pub(crate) id: Uuid,
    // If client is marked as "for developers", some checks will be skipped
    pub(crate) is_dev: bool,
    pub(crate) redirect_uri: Vec<String>,
    pub(crate) title: String,
    pub(crate) allowed_registrations: bool,
}

async fn clients(db: web::Data<Database>) -> Result<Json<Vec<Client>>, Error> {
    let list: Vec<_> = sqlx::query_as!(
        Client,
        // language=PostgreSQL
        r#"
        SELECT id, is_dev, redirect_uri, title, allowed_registrations
        FROM clients
        "#,
    )
    .fetch_all(&db.pool)
    .await
    .map_err(UnexpectedDatabaseError::SqlxError)?
    .into_iter()
    .collect();

    Ok(Json(list))
}

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

#[actix_web::main]
async fn main() -> Result<(), LambdaError> {
    let _ = dotenv::dotenv();
    let db = Database::new(env::var("DATABASE_URL")?, 1);

    let factory = move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/pets/{id}", web::get().to(pets_get))
            .route("/clients", web::get().to(clients))
    };

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    if is_running_on_lambda() {
        run_actix_on_lambda(factory).await?;
    } else {
        HttpServer::new(factory)
            .bind((Ipv4Addr::UNSPECIFIED, 8080))?
            .run()
            .await?;
    }

    Ok(())
}
