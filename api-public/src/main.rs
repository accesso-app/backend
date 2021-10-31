#![deny(warnings)]
#![forbid(unsafe_code)]

use accesso_app::{install_logger, not_found};
use accesso_settings::Settings;
use actix_web::{middleware, web, HttpServer};
use eyre::WrapErr;
use std::sync::Arc;
use tracing_actix_web::TracingLogger;

mod generated;
mod health;
mod lib;
mod routes;

pub static APP_NAME: &str = "accesso-api-public";

#[actix_rt::main]
async fn main() -> eyre::Result<()> {
    let settings = Arc::new(Settings::new("public").wrap_err("failed to parse settings")?);

    if !settings.debug {
    } else {
        color_eyre::install()?;
    }
    dotenv::dotenv().wrap_err("Failed to initialize dotenv")?;

    let _guard = install_logger(APP_NAME.into(), &settings)?;

    let bind_address = settings.server.bind_address();

    if settings.debug {
        tracing::info!("==> api-public running in DEVELOPMENT MODE");
    } else {
        tracing::info!("==> PRODUCTION MODE in api-public");
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
            .service(health::service())
            .default_service(web::route().to(not_found))
            .service(
                generated::api::create()
                    .bind_oauth_token(routes::oauth::token::route)
                    .bind_viewer_get(routes::viewer::get::route),
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

    #[cfg(not(debug_assertions))]
    if settings.use_opentelemetry {
        opentelemetry::global::shutdown_tracer_provider();
    }

    Ok(())
}
