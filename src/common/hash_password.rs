use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2,
};

pub(crate) fn hash_password(origin_password: &str) -> Option<String> {
    let password = origin_password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default().hash_password(password, &salt).ok()?.to_string();
    Some(password_hash)
}

pub(crate) fn verify_password(password: &str, hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(&hash).unwrap();
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
}

