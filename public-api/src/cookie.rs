#[derive(Debug, Clone)]
pub struct SessionCookieConfig {
    pub path: String,
    pub secure: bool,
    pub http_only: bool,
}
