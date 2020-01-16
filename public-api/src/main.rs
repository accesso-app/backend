// https://github.com/actix/actix-web/blob/3a5b62b5502d8c2ba5d824599171bb381f6b1b49/examples/basic.rs

use actix_web::{
    dev::{AppService, Factory, HttpServiceFactory},
    middleware, web, App, FromRequest, HttpRequest, HttpServer, Scope,
};
use serde::Serialize;
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
    root: Scope,
    routes: HashMap<String, Scope>,
}

impl PublicApi {
    pub fn new() -> Self {
        PublicApi {
            root: Scope::new("/"),
            routes: HashMap::new(),
        }
    }
}

impl HttpServiceFactory for PublicApi {
    fn register(mut self, config: &mut AppService) {
        let keys: Vec<String> = self.routes.keys().cloned().collect();

        for key in keys.iter() {
            if let Some(resource) = self.routes.remove(key) {
                self.root = self.root.service(resource);
            }
        }

        self.root.register(config);
    }
}

pub mod components {
    pub mod responses {
        use serde::{Deserialize, Serialize};

        /// User authenticated in Authmenow
        #[derive(Serialize, Deserialize)]
        pub struct UserAuthenticated {
            pub username: Option<String>,
            #[serde(rename = "displayName")]
            pub display_name: Option<String>,
        }

        #[derive(Serialize, Deserialize)]
        pub struct UserAnonymous {}
    }
}

pub mod paths {
    use super::components;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum SessionGetResponse {
        Ok(components::responses::UserAuthenticated),
        NotAuthorized(components::responses::UserAnonymous),
    }

    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum SessionCreateResponse {
        /// Session successfully created
        Ok,
    }
}

impl PublicApi {
    pub fn bind_session_get<F, T, R>(mut self, handler: F) -> Self
    where
        F: Factory<T, R, web::Json<paths::SessionGetResponse>>,
        T: FromRequest + 'static,
        R: Future<Output = web::Json<paths::SessionGetResponse>> + 'static,
    {
        take_mut::take(
            self.routes
                .entry("/session".to_string())
                .or_insert_with(|| web::scope("/session")),
            |scope| scope.route("", web::get().to(handler)),
        );

        self
    }

    pub fn bind_session_create<F, T, R>(mut self, handler: F) -> Self
    where
        F: Factory<T, R, web::Json<paths::SessionCreateResponse>>,
        T: FromRequest + 'static,
        R: Future<Output = web::Json<paths::SessionCreateResponse>> + 'static,
    {
        take_mut::take(
            self.routes
                .entry("/session".to_string())
                .or_insert_with(|| web::scope("/session")),
            |scope| scope.route("", web::post().to(handler)),
        );

        self
    }
}

async fn session_get() -> web::Json<paths::SessionGetResponse> {
    web::Json(paths::SessionGetResponse::Ok(
        components::responses::UserAuthenticated {
            username: Some(String::from("sergeysova")),
            display_name: Some(String::from("🦉")),
        },
    ))
}

async fn session_create() -> web::Json<paths::SessionCreateResponse> {
    web::Json(paths::SessionCreateResponse::Ok)
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
