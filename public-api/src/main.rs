#[macro_use]
extern crate diesel;

use actix_web::{middleware, web, HttpResponse, HttpServer};
use handler::{not_found, AnswerFailure, FailureCode};
use std::sync::RwLock;

pub type App =
    RwLock<authmenow_public_logic::App<services::Database, services::Email, services::Generator>>;

mod cookie;
mod generated;
mod handler;
mod health;
mod routes;
mod services;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let listen_port = std::env::var("LISTEN_PORT").expect("LISTEN_PORT");
    let listen_host = std::env::var("LISTEN_HOST").expect("LISTEN_HOST");
    let connection_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let is_dev = std::env::var("DEV").map(|d| d != "false").unwrap_or(false);

    let sg_api_key = std::env::var("SG_API_KEY").expect("SG_API_KEY");
    let sg_application_host = std::env::var("SG_APPLICATION_HOST").expect("SG_APPLICATION_HOST");
    let sg_email_confirm_url_prefix =
        std::env::var("SG_EMAIL_CONFIRM_URL_PREFIX").expect("SG_EMAIL_CONFIRM_URL_PREFIX");
    let sg_email_confirm_template =
        std::env::var("SG_EMAIL_CONFIRM_TEMPLATE").expect("SG_EMAIL_CONFIRM_TEMPLATE");
    let sg_sender_email = std::env::var("SG_SENDER_EMAIL").expect("SG_SENDER_EMAIL");

    let bind_address = format!("{host}:{port}", host = listen_host, port = listen_port);

    if is_dev {
        println!("==> public-api runned in DEVELOPMENT MODE");
    } else {
        println!("==> PRODUCTION MODE in public-api");
    }

    let db = services::Database::new(connection_url).expect("Failed to create database");
    let generator = services::Generator::new();
    let emailer = services::Email {
        api_key: sg_api_key,
        sender_email: sg_sender_email,
        application_host: sg_application_host,
        email_confirm_url_prefix: sg_email_confirm_url_prefix,
        email_confirm_template: sg_email_confirm_template,
    };

    let session_cookie_config = cookie::SessionCookieConfig {
        http_only: !is_dev,
        secure: !is_dev,
        path: "/".to_owned(),
        name: "session-token".to_owned(),
    };

    let app = authmenow_public_logic::App {
        db,
        emailer,
        generator,
    };

    let app_lock = std::sync::RwLock::new(app);
    let app_data = web::Data::new(app_lock);

    HttpServer::new(move || {
        actix_web::App::new()
            .app_data(app_data.clone())
            .data(session_cookie_config.clone())
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
            .service(health::service())
            .default_service(web::route().to(not_found))
            .service(
                generated::api::AuthmenowPublicApi::new()
                    .bind_oauth_authorize_request(routes::oauth::authorize::route)
                    .bind_register_confirmation(routes::register::confirmation::route)
                    .bind_register_request(routes::register::request::route)
                    .bind_session_create(routes::session::create::route)
                    .bind_session_get(routes::session::get::route),
            )
    })
    .bind(bind_address)?
    .run()
    .await
}
