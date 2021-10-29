// temporary #![deny(warnings)]
#![forbid(unsafe_code)]

mod graphql;
mod schema;

use std::sync::Arc;

use actix_web::web::Data;
use actix_web::{middleware, web, HttpServer};
use eyre::WrapErr;
use tracing_actix_web::TracingLogger;

use accesso_app::install_logger;
use accesso_settings::Settings;

pub static APP_NAME: &str = "accesso-api-admin";

#[actix_rt::main]
async fn main() -> eyre::Result<()> {
    let settings = Arc::new(Settings::new("admin").wrap_err("failed to parse settings")?);

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

    let schema = schema::schema().finish();

    let mut server = HttpServer::new(move || {
        let settings = settings_clone.clone();
        actix_web::App::new()
            .app_data(Data::new(schema.clone()))
            .configure(|config| {
                let settings = settings.clone();
                accesso_app::configure(config, settings)
            })
            .wrap(
                actix_cors::Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allow_any_method()
                    .max_age(3600),
            )
            .wrap(middleware::Compress::default())
            .wrap(TracingLogger::default())
            .default_service(web::route().to(accesso_app::not_found))
            .service(
                web::resource("/graphql")
                    .route(web::post().to(graphql::main))
                    .route(web::get().to(graphql::main)),
            )
            .service(web::resource("/playground").route(web::get().to(graphql::playground)))
            .service(web::resource("/graphiql").route(web::get().to(graphql::graphiql)))
            .service(
                web::resource("/")
                    .guard(actix_web::guard::Get())
                    .guard(actix_web::guard::Header("upgrade", "websocket"))
                    .to(graphql::subscriptions),
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

    server.bind(&bind_address)?.run().await?;

    // This is so that when application is ran locally in debug mode it wouldn't get stuck
    // trying to send data to telemetry collector
    #[cfg(not(debug_assertions))]
    opentelemetry::global::shutdown_tracer_provider();

    Ok(())
}
