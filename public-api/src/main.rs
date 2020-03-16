#[macro_use]
extern crate diesel;

use actix_web::{middleware, web, HttpResponse, HttpServer};
use handler::{not_found, AnswerFailure, FailureCode};

pub type App = authmenow_public_app::App<services::Database, services::Email, services::Generator>;

mod generated;
mod handler;
mod routes;
mod services;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let listen_port = std::env::var("LISTEN_PORT").expect("LISTEN_PORT");
    let connection_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");

    let bind_address = format!("127.0.0.1:{port}", port = listen_port);

    let db = services::Database::new(connection_url).expect("Failed to create database");
    let generator = services::Generator::new();
    let emailer = services::Email::new();

    let app = authmenow_public_app::App {
        db,
        emailer,
        generator,
    };

    HttpServer::new(move || {
        actix_web::App::new()
            .data(app.clone())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .app_data(web::JsonConfig::default().error_handler(|err, _| {
                let error_message = format!("{}", err);
                actix_web::error::InternalError::from_response(
                    err,
                    HttpResponse::BadRequest().json(AnswerFailure {
                        error: FailureCode::InvalidPayload,
                        message: Some(error_message),
                    }),
                )
                .into()
            }))
            .app_data(web::QueryConfig::default().error_handler(|err, _| {
                let error_message = format!("{}", err);
                actix_web::error::InternalError::from_response(
                    err,
                    HttpResponse::BadRequest().json(AnswerFailure {
                        error: FailureCode::InvalidQueryParams,
                        message: Some(error_message),
                    }),
                )
                .into()
            }))
            .wrap(
                middleware::DefaultHeaders::new()
                    // .header("X-Frame-Options", "deny")
                    .header("X-Content-Type-Options", "nosniff")
                    .header("X-XSS-Protection", "1; mode=block"),
            )
            .default_service(web::route().to(not_found))
            .service(
                generated::api::AuthmenowPublicApi::new()
                    .bind_register_request(routes::register_request::route)
                    .bind_register_confirmation(routes::register_confirmation::route),
            )
    })
    .bind(bind_address)?
    .run()
    .await
}
