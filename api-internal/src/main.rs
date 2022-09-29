use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/hey", web::get().to(manual_hello)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
