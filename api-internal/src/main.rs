use std::net::Ipv4Addr;

use actix_web::web::Json;
use actix_web::{self, web, App, Error, HttpServer};
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

async fn manual_hello() -> Result<Json<Pet>, Error> {
    Ok(Json(Pet {
        name: "Dog".into(),
        id: Uuid::new_v4(),
        created_at: OffsetDateTime::now_utc(),
    }))
}

#[actix_web::main]
async fn main() -> Result<(), LambdaError> {
    let factory = || App::new().route("/hello", web::get().to(manual_hello));

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
