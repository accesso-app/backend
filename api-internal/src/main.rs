// #![deny(warnings)]
#![forbid(unsafe_code)]

use accesso_app::Service;
use accesso_core::contracts::{EmailNotification, Repository, SecureGenerator};
use accesso_settings::Settings;
use actix_web::{middleware, web, HttpResponse, HttpServer};
use handler::{not_found, AnswerFailure, FailureCode};
use std::sync::Arc;

mod cookie;
mod generated;
mod handler;
mod health;
mod routes;
mod session;

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let settings = Settings::new("internal").expect("failed to parse settings");
    let bind_address = settings.server.bind_address();

    if settings.debug {
        println!("==> api-internal runned in DEVELOPMENT MODE");
    } else {
        println!("==> PRODUCTION MODE in api-internal");
    }

    let db: Arc<dyn Repository> = Arc::new(
        accesso_db::Database::new(
            settings.database.connection_url(),
            settings.database.pool_size,
        )
        .await
        .expect("Failed to create database"),
    );

    let generator: Arc<dyn SecureGenerator> =
        Arc::new(accesso_core::services::generator::Generator::new());

    let emailer: Arc<dyn EmailNotification> = Arc::new(accesso_core::services::email::Email {
        api_key: settings.sendgrid.api_key,
        sender_email: settings.sendgrid.sender_email,
        application_host: settings.sendgrid.application_host,
        email_confirm_url_prefix: settings.sendgrid.email_confirm_url_prefix,
        email_confirm_template: settings.sendgrid.email_confirm_template,
    });

    let session_cookie_config = cookie::SessionCookieConfig {
        http_only: settings.cookies.http_only,
        secure: settings.cookies.secure,
        path: settings.cookies.path.clone(),
        name: settings.cookies.name.clone(),
    };

    let app = Arc::new(
        accesso_app::App::builder()
            .with_service(Service::from(db))
            .with_service(Service::from(emailer))
            .with_service(Service::from(generator))
            .build(),
    );

    println!("{:?}", std::env::var("CARGO_MANIFEST_DIR"));

    let mut server = HttpServer::new(move || {
        actix_web::App::new()
            .app_data(web::Data::from(app.clone()))
            .app_data(web::Data::new(session_cookie_config.clone()))
            //.wrap(middleware::Compress::default())
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
                routes::session::get::route
                generated::api::create()
                    .bind_oauth_authorize_request(routes::oauth::authorize::route)
                    .bind_register_confirmation(routes::register::confirmation::route)
                    .bind_register_request(routes::register::request::route)
                    .bind_session_create(routes::session::create::route)
                    .bind_session_delete(routes::session::delete::route)
                    .bind_session_get(routes::session::get::route),
            )
    });

    if let Some(workers) = settings.server.workers {
        server = server.workers(workers as usize);
    }
    if let Some(backlog) = settings.server.backlog {
        server = server.backlog(backlog);
    }
    if let Some(keep_alive) = settings.server.keep_alive {
        server = server.keep_alive(actix_http::KeepAlive::Timeout(keep_alive as usize));
    }
    if let Some(client_shutdown) = settings.server.client_shutdown {
        server = server.client_shutdown(client_shutdown);
    }

    server.bind(bind_address)?.run().await?;

    Ok(())
}
