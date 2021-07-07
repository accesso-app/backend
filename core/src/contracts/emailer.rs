#[cfg(feature = "testing")]
use mockall::*;
use serde::Serialize;

#[cfg_attr(feature = "testing", automock)]
pub trait EmailNotification: Send + Sync {
    fn send(&self, email: String, content: EmailMessage) -> bool;
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
