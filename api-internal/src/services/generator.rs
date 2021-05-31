use accesso_core::contracts::SecureGenerator;

lazy_static::lazy_static! {
    static ref WORDS: Vec<&'static str> = {
        let str = include_str!("../../../resources/words.txt");
        str.lines().collect()
    };
}

#[derive(Clone, Default)]
pub struct Generator {}

const TOKEN_LENGTH: u8 = 28;
const TOKEN_LONG_LENGTH: usize = 52;

impl Generator {
    pub fn new() -> Self {
        Self {}
    }
}

impl SecureGenerator for Generator {
    fn secure_words(&self, length: u8) -> String {
        create_words_password(length, "-")
    }

    fn password_hash(&self, password: String) -> (String, Vec<u8>) {
        use sodiumoxide::crypto::pwhash::argon2id13;
        sodiumoxide::init().unwrap();

        let hash = argon2id13::pwhash(
            password.as_bytes(),
            argon2id13::OPSLIMIT_INTERACTIVE,
            argon2id13::MEMLIMIT_INTERACTIVE,
        )
        .unwrap();

        let texthash = std::str::from_utf8(&hash.0).unwrap().to_owned();

        (texthash, hash.0.to_vec())
    }

    fn verify_hash(&self, hash: &[u8], password: &str) -> bool {
        use sodiumoxide::crypto::pwhash::argon2id13;
        sodiumoxide::init().unwrap();

        match argon2id13::HashedPassword::from_slice(&hash) {
            Some(hp) => argon2id13::pwhash_verify(&hp, password.as_bytes()),
            _ => false,
        }
    }

    fn generate_token(&self) -> String {
        random_string(TOKEN_LENGTH as usize)
    }

    fn generate_token_long(&self) -> String {
        random_string(TOKEN_LONG_LENGTH)
    }
}

fn create_words_password(length: u8, separator: &str) -> String {
    use rand::prelude::*;
    let mut rng = rand::thread_rng();

    let mut words = vec![];

    for _ in 0..length {
        let pos = rng.gen_range(0, WORDS.len() - 1);
        words.push(WORDS[pos].to_owned());
    }

    words.join(separator)
}

fn random_string(length: usize) -> String {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .collect()
}
