#[derive(Debug, Clone)]
pub struct SessionCookieConfig {
    pub name: String,
    pub path: String,
    pub secure: bool,
    pub http_only: bool,
}
