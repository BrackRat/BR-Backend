use std::env;

use chrono::Days;
use jsonwebtoken::{Algorithm, decode, DecodingKey, encode, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub id: i32,
    pub exp: usize,
}

const EXP_DELTA: u64 = 15;

fn check_expire(exp: usize) -> bool {
    let check = exp > exp_duration();
    return check;
}

fn exp_duration() -> usize {
    let exp_time = chrono::Local::now().checked_add_days(Days::new(EXP_DELTA)).unwrap().timestamp() as usize;
    return exp_time;
}


pub fn generate_jwt(id: i32) -> String {
    let binding = env::var("JWT_SECRET")
        .expect("JWT_SECRET environment variable not found");
    let jwt_screct: &[u8] = binding
        .as_bytes();

    let my_claims = Claims {
        id,
        exp: exp_duration(),
    };
    let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret(jwt_screct.as_ref())).unwrap();
    let bearer = format!("Bearer {}", token);
    bearer
}

pub fn verify_jwt(token: String) -> Option<Claims> {
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