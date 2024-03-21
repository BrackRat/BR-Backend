use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
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
mod routes;

use routes::user::*;

#[get("/")]
async fn hello() -> impl Responder {
    let response = serde_json::json!({
        "code": 200,
        "msg": "BR COMMON BACKEND"
    });
    HttpResponse::Ok().json(response)
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
                            .service(get_user_detail)
                            .service(user_change_password)
                    )
            )
    })
        .bind((ip, port))?
        .run()
        .await
}