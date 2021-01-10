pub use config;
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub database: Database,
    pub cookies: Cookies,
    pub server: Server,
    pub sendgrid: SendGrid,
}

#[derive(Debug, Deserialize)]
pub struct Cookies {
    pub http_only: bool,
    pub secure: bool,
    pub path: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: i32,
    pub database: String,
    pub pool_size: u32,
}

#[derive(Debug, Deserialize)]
pub struct SendGrid {
    pub api_key: String,
    /// with port
    pub application_host: String,
    /// `"/register/confirm-"`
    pub email_confirm_url_prefix: String,
    /// Template ID
    pub email_confirm_template: String,
    /// `no-reply@accesso.sova.dev`
    pub sender_email: String,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub port: u16,
    pub host: String,
    pub workers: Option<u16>,
    pub backlog: Option<i32>,
    pub keep_alive: Option<u16>,
    pub client_shutdown: Option<u64>,
}

impl Settings {
    pub fn new<N: AsRef<str>>(api_name: N) -> Result<Self, ConfigError> {
        let mut config = Config::new();
        let mode = env::var("ACCESSO_MODE").unwrap_or("development".to_owned());
        let api = api_name.as_ref();

        // Load environment config
        let env = match mode.as_ref() {
            "development" => "development",
            "test" => "test",
            "production" => "production",
            mode => panic!("invalid ACCESSO_MODE {}", mode),
        };

        config.merge(File::with_name("config/default"))?;

        let files = vec![
            format!("config/default-{}", env), // config/default-production.toml
            format!("config/{}", api),         // config/internal.toml
            format!("config/{}-{}", api, env), // config/internal-production.toml
            // locals
            ".config".to_owned(),               // .config.toml
            format!(".config-{}", env),         // .config-production.toml
            format!(".config-{}", api),         // .config-internal.toml
            format!(".config-{}-{}", api, env), // .config-internal-production.toml
        ];

        for path in files.iter() {
            config.merge(File::with_name(path).required(false))?;
        }

        // Add in settings from the environment (with a prefix of ACCESSO)
        // Eg.. `ACCESSO_DEBUG=true ./target/app` would set the `debug` key
        // Note: we need to use double underscore here, because otherwise variables containing
        //       underscore cant be set from environmnet.
        // https://github.com/mehcode/config-rs/issues/73
        config.merge(Environment::with_prefix("ACCESSO").separator("__"))?;

        config.try_into()
    }
}

impl Server {
    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl Database {
    pub fn connection_url(&self) -> String {
        format!(
            "postgres://{user}:{password}@{host}:{port}/{db}",
            user = self.user,
            password = self.password,
            host = self.host,
            port = self.port,
            db = self.database,
        )
    }
}
