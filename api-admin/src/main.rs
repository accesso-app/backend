// temporary #![deny(warnings)]
#![forbid(unsafe_code)]

mod routes;
mod services;
mod accesso;
mod generated;

use accesso_settings::Settings;

use accesso_app::{install_logger, not_found, Service};
use accesso_core::contracts::{EmailNotification, Repository, SecureGenerator};
use actix_swagger::{Answer, StatusCode};
use actix_web::web::Data;
use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use eyre::WrapErr;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing_actix_web::TracingLogger;

use shrinkwraprs::Shrinkwrap;
use url::Url;
use actix_cors::Cors;

#[derive(Debug, Shrinkwrap, Clone)]
#[shrinkwrap(mutable)]
pub struct AccessoUrl(pub Url);

pub static APP_NAME: &str = "accesso-api-admin";

#[actix_rt::main]
async fn main() -> eyre::Result<()> {
    let settings = Arc::new(Settings::new("admin").wrap_err("failed to parse settings")?);

    let client = create_request_client(&settings)?;
    let accesso_url = Arc::new(AccessoUrl(Url::parse(&settings.accesso.url)?));
    let client_clone = client.clone();

    if !settings.debug {
    } else {
        color_eyre::install()?;
    }
    dotenv::dotenv().wrap_err("Failed to initialize dotenv")?;

    let _guard = install_logger(APP_NAME.into(), &settings)?;

    let bind_address = settings.server.bind_address();

    if settings.debug {
        tracing::info!("==> api-admin running in DEVELOPMENT MODE");
    } else {
        tracing::info!("==> PRODUCTION MODE in api-admin");
    }

    let settings_clone = settings.clone();

    let mut server = HttpServer::new(move || {
        let settings = settings_clone.clone();
        let client = client_clone.clone();
        let accesso_url = accesso_url.clone();

        App::new()
            .configure(|config| {
                let settings = settings.clone();
                accesso_app::configure(config, settings)
            })
            .service(generated::api::create()
                .bind_auth_params(routes::oauth::auth_params::route)
                .bind_auth_done(routes::oauth::auth_done::route)
                .bind_session_get(routes::session::get::route)
            )
            .wrap(Cors::permissive())
            .wrap(middleware::Compress::default())
            .wrap(
                middleware::DefaultHeaders::new()
                    .header("X-Frame-Options", "deny")
                    .header("X-Content-Type-Options", "nosniff")
                    .header("X-XSS-Protection", "1; mode=block"),
            )
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(client))
            .app_data(web::Data::from(accesso_url))
            .default_service(web::route().to(not_found))
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

    server.bind(&bind_address)?.run().await?;

    Ok(())
}

pub fn create_request_client(config: &Settings) -> Result<reqwest::Client, eyre::Report> {
    let mut builder = reqwest::ClientBuilder::new();

    if !config.accesso.ssl_validate {
        tracing::warn!(
            "!!! SSL validation is disabled in config, check if this is what you REALLY want !!!"
        );
        builder = builder.danger_accept_invalid_certs(true);
    }

    builder.build().wrap_err("Could not create http client!")
}
