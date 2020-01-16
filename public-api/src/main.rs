// https://github.com/actix/actix-web/blob/3a5b62b5502d8c2ba5d824599171bb381f6b1b49/examples/basic.rs

use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct AnswerFailure {
    code: i32,
    message: String,
}

async fn not_found(_req: HttpRequest) -> web::Json<AnswerFailure> {
    web::Json(AnswerFailure {
        code: 404,
        message: "Route not found".to_string(),
    })
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::DefaultHeaders::new().header("X-Authmenow", "0.1"))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .default_service(web::route().to(not_found))
    })
    .bind("127.0.0.1:9000")?
    .workers(1)
    .run()
    .await
}
