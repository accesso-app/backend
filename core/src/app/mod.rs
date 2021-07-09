pub mod oauth;
pub mod registrator;
pub mod session;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Service with name: {0} does not exist in app")]
    ServiceDoesNotExist(&'static str),
}
