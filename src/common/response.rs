use actix_web::{HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub enum ResponseStatus<'a> {
    Success,
    BadRequest(Option<&'a str>),
    Unauthorized(Option<&'a str>),
    NotFound(Option<&'a str>),
    InternalServerError(Option<&'a str>),
}

pub fn generate_response<'a, T>(status: ResponseStatus<'a>, data: Option<T>) -> impl Responder + 'a
    where
        T: Serialize,
{
    let msg = match &status {
        ResponseStatus::Success => "OK",
        ResponseStatus::BadRequest(Some(msg)) | ResponseStatus::Unauthorized(Some(msg)) | ResponseStatus::NotFound(Some(msg)) | ResponseStatus::InternalServerError(Some(msg)) => msg,
        ResponseStatus::BadRequest(None) => "Bad Request",
        ResponseStatus::Unauthorized(None) => "Unauthorized",
        ResponseStatus::NotFound(None) => "Not Found",
        ResponseStatus::InternalServerError(None) => "Internal Server Error",
    };

    let json_response = serde_json::json!({
        "code": match status {
            ResponseStatus::Success => 200,
            ResponseStatus::BadRequest(_) => 400,
            ResponseStatus::Unauthorized(_) => 401,
            ResponseStatus::NotFound(_) => 404,
            ResponseStatus::InternalServerError(_) => 500,
        },
        "msg": msg,
        "data": data,
    });

    HttpResponse::Ok().json(json_response)
}


pub fn response<T: Serialize>(result: Result<T, ResponseStatus>) -> impl Responder + '_ {
    match result {
        Ok(data) => {
            generate_response(ResponseStatus::Success, Some(data))
        }
        Err(e) => {
            generate_response(e, None)
        }
    }
}