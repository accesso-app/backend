// https://github.com/actix/actix-web/blob/3a5b62b5502d8c2ba5d824599171bb381f6b1b49/examples/basic.rs

use actix_web::{
    dev::{AppService, Factory, HttpServiceFactory},
    middleware, web, App, FromRequest, HttpRequest, HttpResponse, HttpServer, Resource, Responder,
    Route, Scope,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::future::Future;

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

struct PublicApi {
    scope: Scope,
    routes: HashMap<String, Resource>,
}

impl PublicApi {
    pub fn new() -> Self {
        PublicApi {
            scope: Scope::new("/"),
            routes: HashMap::new(),
        }
    }
}

impl HttpServiceFactory for PublicApi {
    fn register(self, config: &mut AppService) {
        for (path, route) in self.routes.iter() {
            self.scope.service(*route);
        }
        self.scope.register(config);
    }
}

#[derive(Serialize, Deserialize)]
struct UserAuthenticated {
    id: i32,
}
#[derive(Serialize, Deserialize)]
struct UserAnonymous {}

type SessionGetResponse = Result<UserAuthenticated, UserAnonymous>;

impl PublicApi {
    pub fn bind_session_get<F, T, R>(mut self, handler: F) -> Self
    where
        F: Factory<T, R, web::Json<SessionGetResponse>>,
        T: FromRequest + 'static,
        R: Future<Output = web::Json<SessionGetResponse>> + 'static,
    {
        // let resource = self
        //     .routes
        //     .entry("/session".to_string())
        //     .or_insert(web::resource("/session"));

        // let value = self.routes.remove(&"/session".to_string());
        // let resource = match value {
        //     Some(resource) => resource.route(web::get().to(handler)),
        //     None => web::resource("/session").route(web::get().to(handler)),
        // };

        // self.routes.insert("/session".to_string(), resource);

        self
    }

    pub fn bind_session_create<F, T, R>(mut self, handler: F) -> Self
    where
        F: Factory<T, R, web::Json<Nothing>>,
        T: FromRequest + 'static,
        R: Future<Output = web::Json<Nothing>> + 'static,
    {
        let value = self.routes.remove(&"/session".to_string());
        let resource = match value {
            Some(resource) => resource.route(web::post().to(handler)),
            None => web::resource("/session").route(web::get().to(handler)),
        };

        self.routes.insert("/session".to_string(), resource);

        self
    }
}

async fn session_get() -> web::Json<SessionGetResponse> {
    web::Json(Ok(UserAuthenticated { id: 1 }))
}

#[derive(Serialize)]
struct Nothing {
    i: i32,
}

async fn session_create() -> web::Json<Nothing> {
    web::Json(Nothing { i: 12 })
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
                PublicApi::new()
                    .bind_session_get(session_get)
                    .bind_session_create(session_create),
            )
    })
    .bind("127.0.0.1:9005")?
    .workers(1)
    .run()
    .await
}
