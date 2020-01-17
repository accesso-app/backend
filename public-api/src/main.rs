// https://github.com/actix/actix-web/blob/3a5b62b5502d8c2ba5d824599171bb381f6b1b49/examples/basic.rs

use actix_web::{
    dev::{AppService, Factory, HttpServiceFactory},
    http::StatusCode,
    middleware, web, App, FromRequest, HttpRequest, HttpServer, Responder, Scope,
};
use serde::Serialize;
use std::collections::HashMap;
use std::future::Future;

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
    use actix_http::Response;
    use actix_web::{http::StatusCode, Error, HttpRequest, Responder};
    use futures::future::{err, ok, Ready};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum SessionGetResponse {
        Ok(components::responses::UserAuthenticated),
        NotAuthorized(components::responses::UserAnonymous),
    }

    impl Responder for SessionGetResponse {
        type Error = Error;
        type Future = Ready<Result<Response, Error>>;

        fn respond_to(self, _: &HttpRequest) -> Self::Future {
            let body = match serde_json::to_string(&self) {
                Ok(body) => body,
                Err(e) => return err(e.into()),
            };

            ok(Response::build(match self {
                SessionGetResponse::Ok(_) => StatusCode::OK,
                SessionGetResponse::NotAuthorized(_) => StatusCode::UNAUTHORIZED,
            })
            .content_type("application/json")
            .body(body))
            // how to set headers from handler?
        }
    }

    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum SessionCreateResponse {
        /// Session successfully created
        Ok,
    }

    impl Responder for SessionCreateResponse {
        type Error = Error;
        type Future = Ready<Result<Response, Error>>;

        fn respond_to(self, _: &HttpRequest) -> Self::Future {
            let body = match serde_json::to_string(&self) {
                Ok(body) => body,
                Err(e) => return err(e.into()),
            };

            ok(Response::build(match self {
                SessionCreateResponse::Ok => StatusCode::OK,
            })
            .content_type("application/json")
            .body(body))
        }
    }

    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum SessionDeleteResponse {
        /// Session successfully deleted
        Ok,
    }

    impl Responder for SessionDeleteResponse {
        type Error = Error;
        type Future = Ready<Result<Response, Error>>;

        fn respond_to(self, _: &HttpRequest) -> Self::Future {
            let body = match serde_json::to_string(&self) {
                Ok(body) => body,
                Err(e) => return err(e.into()),
            };

            ok(Response::build(match self {
                SessionDeleteResponse::Ok => StatusCode::OK,
            })
            .content_type("application/json")
            .body(body))
        }
    }
}

impl PublicApi {
    pub fn bind_session_get<F, T, R>(mut self, handler: F) -> Self
    where
        F: Factory<T, R, paths::SessionGetResponse>,
        T: FromRequest + 'static,
        R: Future<Output = paths::SessionGetResponse> + 'static,
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
        F: Factory<T, R, paths::SessionCreateResponse>,
        T: FromRequest + 'static,
        R: Future<Output = paths::SessionCreateResponse> + 'static,
    {
        take_mut::take(
            self.routes
                .entry("/session".to_string())
                .or_insert_with(|| web::scope("/session")),
            |scope| scope.route("", web::post().to(handler)),
        );

        self
    }

    pub fn bind_session_delete<F, T, R>(mut self, handler: F) -> Self
    where
        F: Factory<T, R, paths::SessionDeleteResponse>,
        T: FromRequest + 'static,
        R: Future<Output = paths::SessionDeleteResponse> + 'static,
    {
        take_mut::take(
            self.routes
                .entry("/session".to_string())
                .or_insert_with(|| web::scope("/session")),
            |scope| scope.route("", web::delete().to(handler)),
        );

        self
    }
}

async fn session_get() -> paths::SessionGetResponse {
    paths::SessionGetResponse::Ok(components::responses::UserAuthenticated {
        username: Some(String::from("sergeysova")),
        display_name: Some(String::from("🦉")),
    })
}

async fn session_create() -> paths::SessionCreateResponse {
    paths::SessionCreateResponse::Ok
}

async fn session_delete() -> paths::SessionDeleteResponse {
    paths::SessionDeleteResponse::Ok
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
                    .bind_session_create(session_create)
                    .bind_session_delete(session_delete),
            )
    })
    .bind("127.0.0.1:9005")?
    .workers(1)
    .run()
    .await
}
