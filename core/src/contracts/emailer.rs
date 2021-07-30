use async_trait::async_trait;
#[cfg(feature = "testing")]
use mockall::*;
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum SendEmailError {
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
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
