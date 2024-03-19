use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    name: String,
    exp: usize,
}


pub(crate) fn generate_jwt(name: String) -> String {
    let binding = env::var("JWT_SECRET")
        .expect("JWT_SECRET environment variable not found");
    let jwt_screct: &[u8] = binding
        .as_bytes();


    let my_claims = Claims {
        name,
        exp: (chrono::Local::now() + chrono::Duration::days(15)).timestamp() as usize
    };
    let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret(jwt_screct.as_ref())).unwrap();
    return token;
}

pub(crate) fn verify_jwt(token: String) -> bool {
    let binding = env::var("JWT_SECRET")
        .expect("JWT_SECRET environment variable not found");
    let jwt_screct: &[u8] = binding
        .as_bytes();
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(&token, &DecodingKey::from_secret(jwt_screct.as_ref()), &validation).unwrap();
    return token_data.claims.name == token_data.claims.name;
}