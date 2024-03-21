use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use std::env;
use std::fmt::format;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub(crate) struct Claims {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) exp: usize,
}

const EXP_DELTA: i64 = 15i64;

fn check_expire(exp: usize) -> bool {
    return exp > (chrono::Local::now() + chrono::Duration::days(EXP_DELTA)).timestamp() as usize;
}


pub(crate) fn generate_jwt(id: i32, name: String) -> String {
    let binding = env::var("JWT_SECRET")
        .expect("JWT_SECRET environment variable not found");
    let jwt_screct: &[u8] = binding
        .as_bytes();

    let my_claims = Claims {
        id,
        name,
        exp: (chrono::Local::now() + chrono::Duration::days(EXP_DELTA)).timestamp() as usize,
    };
    let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret(jwt_screct.as_ref())).unwrap();
    // jsonwebtoken::encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET))
    //     .map(|s| format!("Bearer {}", s))
    //     .unwrap()
    let bearer = format!("Bearer {}", token);
    bearer
}

pub(crate) fn verify_jwt(token: String) -> Option<Claims> {
    let binding = env::var("JWT_SECRET")
        .expect("JWT_SECRET environment variable not found");
    let jwt_screct: &[u8] = binding
        .as_bytes();
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(&token, &DecodingKey::from_secret(jwt_screct.as_ref()), &validation);
    if token_data.is_err() {
        return None;
    }
    let data = token_data.unwrap().claims;
    if check_expire(data.exp.clone()) {
        return None;
    }
    Some(data)
}