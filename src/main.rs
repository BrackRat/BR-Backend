use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
// use serde::{Serialize, Deserialize};
use serde_json;
use actix_web::middleware::Logger;
use actix_web::web::scope;
use env_logger::Env;

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
use crate::common::jwt::verify_jwt;


#[get("/")]
async fn hello() -> impl Responder {
    let response = serde_json::json!({
        "code": 200,
        "msg": "BR COMMON BACKEND"
    });
    HttpResponse::Ok().json(response)
}


#[post("/register")]
async fn user_register(client: web::Data<PrismaClient>, body: web::Json<request::UserRegisterReq>) -> impl Responder {
    let result = controller::user::register_user(client, body.name.clone(), body.password.clone()).await;
    match result {
        Some(user) => {
            generate_response(ResponseStatus::Success, Some(serde_json::json!(
                {
                    "id": user.id
                }
            )), None)
        }
        None => {
            generate_response(ResponseStatus::BadRequest, None, Some("Username already exists"))
        }
    }
}

#[post("/login")]
async fn user_login(client: web::Data<PrismaClient>, body: web::Json<request::UserLoginReq>) -> impl Responder {
    let result = controller::user::login_user(client, body.name.clone(), body.password.clone()).await;
    match result {
        Some(token) => {
            generate_response(ResponseStatus::Success, Some(serde_json::json!(
                {
                    "token": token
                }
            )), None)
        }
        None => {
            generate_response(ResponseStatus::BadRequest, None, Some("Username or Password Wrong"))
        }
    }
}

#[post("/change_password")]
async fn user_change_password(client: web::Data<PrismaClient>, body: web::Json<request::UserChangePasswordReq>) -> impl Responder {
    let result = controller::user::change_password(client, body.name.clone(), body.old_password.clone(), body.new_password.clone()).await;
    match result {
        true => {
            generate_response(ResponseStatus::Success, Some(serde_json::json!(
                {
                    "well": "fk"
                }
            )), None)
        }
        false => {
            generate_response(ResponseStatus::BadRequest, None, None)
        }
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = web::Data::new(PrismaClient::_builder().build().await.unwrap());
    let ip = "127.0.0.1";
    let port = 5050;

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    println!("Listening on {}:{}", ip, port);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(client.clone())
            .service(
                scope("/api")
                    .service(hello)
                    .service(
                        scope("/user")
                            .service(user_login)
                            .service(user_register)
                            .service(user_change_password)
                    )
            )
    })
        .bind((ip, port))?
        .run()
        .await
}