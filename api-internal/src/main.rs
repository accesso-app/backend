use actix_web::{self, web, App, HttpResponse, HttpServer, Responder};
use lambda_web::{is_running_on_lambda, run_actix_on_lambda, LambdaError};
use std::net::Ipv4Addr;

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
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
