#[cfg(feature = "testing")]
use mockall::*;

#[cfg_attr(feature = "testing", automock)]
pub trait SecureGenerator: Send + Sync {
    fn secure_words(&self, length: u8) -> String;
    fn confirmation_code(&self) -> String {
        self.secure_words(4)
    }

    fn generate_token(&self) -> String;
    fn generate_token_long(&self) -> String;

    fn password_hash(&self, password: String) -> (String, Vec<u8>);
    fn verify_hash(&self, hash: &[u8], password: &str) -> bool;
}
