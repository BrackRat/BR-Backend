use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
// use serde::{Serialize, Deserialize};
use serde_json;


#[allow(warnings, unused)]
mod db;

use db::*;

// use prisma_client_rust::Direction;
mod utils;
mod controller;
mod common;

use common::response::generate_response;
use common::response::ResponseStatus;
use common::request;


#[get("/")]
async fn hello() -> impl Responder {
    let response = serde_json::json!({
        "code": 200,
        "msg": "BR COMMON BACKEND"
    });
    HttpResponse::Ok().json(response)
}


#[post("/user/register")]
async fn user_register(client: web::Data<PrismaClient>, body: web::Json<request::UserRegisterReq>) -> impl Responder {
    let result = controller::user::register_user(client, body.name.clone(), body.password.clone()).await;
    match result {
        Some(user) => {
            generate_response(ResponseStatus::Success, Some(user), None)
        }
        None => {
            generate_response(ResponseStatus::BadRequest, None, Some("Username already exists"))
        }
    }
}

#[post("/user/login")]
async fn user_login(client: web::Data<PrismaClient>, body: web::Json<request::UserLoginReq>) -> impl Responder {
    let result = controller::user::login_user(client, body.name.clone(), body.password.clone()).await;
    match result {
        Some(token) => {
            generate_response(ResponseStatus::Success, Some(serde_json::json!(
                {
                    "token": token
                }
            )), Some("success"))
        }
        None => {
            generate_response(ResponseStatus::BadRequest, None, Some("Username or Password Wrong"))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = web::Data::new(PrismaClient::_builder().build().await.unwrap());
    let ip = "127.0.0.1";
    let port = 5050;

    println!("Listening on {}:{}", ip, port);

    HttpServer::new(move || {
        App::new()
            .app_data(client.clone())
            .service(hello)
            .service(user_login)
            .service(user_register)
    })
        .bind((ip, port))?
        .run()
        .await
}