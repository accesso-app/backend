#[cfg(test)]
use mockall::*;
use serde::Serialize;

#[cfg_attr(test, automock)]
pub trait EmailNotification {
    fn send(&mut self, email: String, content: EmailMessage) -> bool;
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
