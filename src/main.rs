use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};
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

#[get("/")]
async fn hello() -> impl Responder {
    let response = serde_json::json!({
        "code": 200,
        "msg": "BR COMMON BACKEND"
    });
    HttpResponse::Ok().json(response)
}

#[derive(Debug, Serialize, Deserialize)]
struct UserRegisterReq {
    name: String,
    password: String,
}

#[post("/user/register")]
async fn register(client: web::Data<PrismaClient>, body: web::Json<UserRegisterReq>) -> impl Responder {
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

// #[post("/update")]
// async fn update(client: web::Data<PrismaClient>, body: web::Json<UpdateReqBody>) -> impl Responder {
//     let update = client
//         .history()
//         .create(
//             body.timestamp,
//             body.co2,
//             body.tvoc,
//             vec![],
//         )
//         .exec()
//         .await
//         .unwrap();
//     HttpResponse::Ok().json(update)
// }

// #[get("/status")]
// async fn get_latest(client: web::Data<PrismaClient>) -> impl Responder {
//     let latest = client
//         .history()
//         .find_first(vec![])
//         .order_by(history::timestamp::order(Direction::Desc))
//         .exec()
//         .await
//         .unwrap();
//     match latest {
//         None => { HttpResponse::Ok().json("No data found.") }
//         Some(data) => {
//             let response = serde_json::json!({
//             "code": 200,
//             "data": {
//                 "co2": data.co_2,
//                 "tvoc": data.tvoc,
//                 "time": time_format(data.timestamp),
//                 "id": data.id
//                 }
//             });
//
//             HttpResponse::Ok().json(response)
//         }
//     }
// }
//
//
// #[get("/history")]
// async fn get_history_chart(client: web::Data<PrismaClient>) -> impl Responder {
//     let latest = client
//         .history()
//         .find_many(vec![])
//         .order_by(history::timestamp::order(Direction::Desc))
//         .take(8640)
//         .exec()
//         .await
//         .unwrap();
//
//     let history_data = latest
//         .iter()
//         .map(|history| {
//             (time_format(history.timestamp), history.co_2, history.tvoc)
//         })
//         .collect::<Vec<(String, i32, i32)>>();
//
//     let response = serde_json::json!({
//         "code": 200,
//         "data": {
//             "history_data": history_data
//         }
//     });
//
//     HttpResponse::Ok().json(response)
// }

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
                .service(register)
            // .service(update)
        })
            .bind((ip, port))?
            .run()
            .await
    }