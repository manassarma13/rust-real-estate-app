use argon2::{self, Config};

pub fn hash_password(password: &str) -> String {
    let salt = b"randomsalt";
    argon2::hash_encoded(password.as_bytes(), salt, &Config::default()).unwrap()
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    argon2::verify_encoded(hash, password.as_bytes()).unwrap()
}
