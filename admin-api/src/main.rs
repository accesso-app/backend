use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Serialize)]
struct Demo {
    text: String,
}

#[derive(Deserialize)]
struct HelloPath {
    id: Uuid,
}

async fn demo(pool: web::Data<DbPool>, name: web::Path<HelloPath>) -> web::Json<Demo> {
    let name = name.id.to_owned();
    let _conn = pool.get().expect("could not get connection from pool");

    web::Json(Demo {
        text: format!("Hello {}", name),
    })
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,diesel=debug");
    env_logger::init();
    dotenv::dotenv().ok();

    let listen_port = std::env::var("LISTEN_PORT").expect("LISTEN_PORT");
    let connection_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(connection_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    let bind = format!("127.0.0.1:{}", listen_port);

    let server = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/hello/{id}").route(web::get().to(demo)))
    })
    .bind(&bind)?;

    println!("Starting server on port {}", listen_port);

    server.run().await
}
