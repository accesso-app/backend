#![deny(warnings)]
#![forbid(unsafe_code)]

use std::sync::Arc;

use actix_web::{middleware, web, HttpServer};
use eyre::WrapErr;
use tracing_actix_web::TracingLogger;

use accesso_app::install_logger;
use accesso_settings::Settings;
use routes::account;

mod generated;
mod routes;
mod session;

pub static APP_NAME: &str = "accesso-api-internal";

#[actix_rt::main]
async fn main() -> eyre::Result<()> {
    let settings = Arc::new(Settings::new("internal").wrap_err("failed to parse settings")?);

    if !settings.debug {
    } else {
        color_eyre::install()?;
    }
    dotenv::dotenv().wrap_err("Failed to initialize dotenv")?;

    let _guard = install_logger(APP_NAME.into(), &settings)?;

    let bind_address = settings.server.bind_address();

    if settings.debug {
        tracing::info!("==> api-internal running in DEVELOPMENT MODE");
    } else {
        tracing::info!("==> PRODUCTION MODE in api-internal");
    }

    let settings_clone = settings.clone();

    let mut server = HttpServer::new(move || {
        let settings = settings_clone.clone();
        actix_web::App::new()
            .configure(|config| {
                let settings = settings.clone();
                accesso_app::configure(config, settings)
            })
            .wrap(middleware::Compress::default())
            .wrap(
                middleware::DefaultHeaders::new()
                    // .header("X-Frame-Options", "deny")
                    .header("X-Content-Type-Options", "nosniff")
                    .header("X-XSS-Protection", "1; mode=block"),
            )
            .wrap(TracingLogger::default())
            .default_service(web::route().to(accesso_app::not_found))
            .service(
                generated::api::create()
                    .bind_oauth_authorize_request(routes::oauth::authorize::route)
                    .bind_register_confirmation(routes::register::confirmation::route)
                    .bind_register_request(routes::register::request::route)
                    .bind_session_create(routes::session::create::route)
                    .bind_session_delete(routes::session::delete::route)
                    .bind_session_get(routes::session::get::route)
                    .bind_account_edit(account::edit::route)
                    .bind_application_get(routes::application::get::route),
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

    // This is so that when application is ran locally in debug mode it wouldn't get stuck
    // trying to send data to telemetry collector
    #[cfg(not(debug_assertions))]
    opentelemetry::global::shutdown_tracer_provider();

    Ok(())
}
