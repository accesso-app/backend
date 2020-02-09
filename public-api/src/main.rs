// https://github.com/actix/actix-web/blob/3a5b62b5502d8c2ba5d824599171bb381f6b1b49/examples/basic.rs

use actix_swagger::{Answer, ContentType};
use actix_web::{
    http::{Cookie, StatusCode},
    middleware, web, App, HttpRequest, HttpServer, Responder,
};
use serde::Serialize;

mod generated;

#[derive(Debug, Serialize)]
struct AnswerFailure {
    code: i32,
    message: String,
}

async fn not_found(_req: HttpRequest) -> impl Responder {
    web::Json(AnswerFailure {
        code: 404,
        message: "Route not found".to_string(),
    })
    .with_status(StatusCode::NOT_FOUND)
}

async fn session_get(
    a: web::Json<generated::paths::SessionGetResponse>,
    b: HttpRequest,
) -> Answer<'static, generated::paths::SessionGetResponse> {
    use generated::components::responses::UserAuthenticated;
    use generated::paths::SessionGetResponse;

    println!("{:#?}", a);
    println!("{:#?}", b.headers());

    SessionGetResponse::Ok(UserAuthenticated {
        username: Some(String::from("sergeysova")),
        display_name: Some(String::from("🦉")),
    })
    .answer()
    .header("x-csrf-token".to_string(), "DEEEEEEEEMO")
    .cookie(
        Cookie::build("CSRF-Token", "HopHey")
            .secure(true)
            .http_only(true)
            .finish(),
    )
    .cookie(Cookie::build("demo", "value").finish())
}

async fn session_create() -> Answer<'static, generated::paths::SessionCreateResponse> {
    generated::paths::SessionCreateResponse::Ok.answer()
}

async fn session_delete() -> Answer<'static, generated::paths::SessionDeleteResponse> {
    generated::paths::SessionDeleteResponse::Ok.answer()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .default_service(web::route().to(not_found))
            .service(
                generated::api::AuthmenowPublicApi::new()
                    .bind_session_get(session_get)
                    .bind_session_create(session_create)
                    .bind_session_delete(session_delete),
            )
    })
    .bind("127.0.0.1:9005")?
    .workers(1)
    .run()
    .await
}
