use argon2::{self, Config};

use rand::Rng;

lazy_static::lazy_static! {
  pub static ref SECRET_KEY: String = std::env::var("SECRET_KEY").unwrap();
}

pub fn hash_password(password: &str) -> Result<String, argon2::Error> {
    let config = Config {
        secret: SECRET_KEY.as_bytes(),
        ..Default::default()
    };

    let salt: [u8; 32] = rand::thread_rng().gen();

    argon2::hash_encoded(password.as_bytes(), &salt, &config)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, argon2::Error> {
    argon2::verify_encoded_ext(hash, password.as_bytes(), SECRET_KEY.as_bytes(), &[])
}
