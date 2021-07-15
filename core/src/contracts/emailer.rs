use async_trait::async_trait;
#[cfg(feature = "testing")]
use mockall::*;
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum SendEmailError {
    #[error("Isahc error: {0}")]
    IsahcError(#[from] isahc::Error),
    #[error("Http error: {0}")]
    HttpError(#[from] isahc::http::Error),
    #[error("Serialize json error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error(transparent)]
    Unexpected(#[from] eyre::Report),
}

#[cfg_attr(feature = "testing", automock)]
#[async_trait]
pub trait EmailNotification: Send + Sync {
    async fn send(&self, email: String, content: EmailMessage) -> Result<(), SendEmailError>;
}

#[derive(Debug, Serialize)]
pub enum EmailMessage {
    RegisterConfirmation {
        code: String,
    },
    RegisterFinished {
        first_name: String,
        last_name: String,
    },
}
