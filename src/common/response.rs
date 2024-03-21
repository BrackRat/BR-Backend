use actix_web::{HttpResponse, Responder};
use serde::Serialize;

#[allow(warnings)]
#[derive(Serialize)]
pub enum ResponseStatus {
    Success,
    BadRequest,
    Unauthorized,
    NotFound,
    InternalServerError,
}

pub fn generate_response<T>(status: ResponseStatus, data: Option<T>, custom_msg: Option<&str>) -> impl Responder where T: Serialize {
    let default_msg = match status {
        ResponseStatus::Success => "OK",
        ResponseStatus::BadRequest => "Bad Request",
        ResponseStatus::NotFound => "Not Found",
        ResponseStatus::Unauthorized => "Unauthorized",
        ResponseStatus::InternalServerError => "Internal Server Error",
    };

    let msg = custom_msg.unwrap_or(default_msg);

    let json_response = serde_json::json!({
        "code": match status {
            ResponseStatus::Success => 200,
            ResponseStatus::BadRequest => 400,
            ResponseStatus::Unauthorized => 401,
            ResponseStatus::NotFound => 404,
            ResponseStatus::InternalServerError => 500,
        },
        "msg": msg,
        "data": data,
    });

    HttpResponse::Ok().json(json_response)
}