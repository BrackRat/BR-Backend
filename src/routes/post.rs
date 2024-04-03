use actix_web::{get, post, Responder, web};
use crate::common::auth::UserData;
use crate::models::post::*;
use crate::common::response::{generate_response, ResponseStatus};
use crate::controller;
use crate::prisma::PrismaClient;

#[post("/create")]
pub async fn create_post(client: web::Data<PrismaClient>, body: web::Json<PostCreateReq>, user: UserData) -> impl Responder {
    let result = controller::post::create_post(client, body.title.clone(), &body.content, user.id).await;
    match result {
        Some(post) => {
            generate_response(ResponseStatus::Success, Some(serde_json::json!(
                {
                    "postId": post.id
                }
            )))
        }
        None => {
            generate_response(ResponseStatus::BadRequest(None), None)
        }
    }
}

#[get("/{page}/{size}")]
pub async fn get_posts(client: web::Data<PrismaClient>, page: web::Path<(i64, i64)>) -> impl Responder {
    let result = controller::post::get_posts(client, page.0, page.1).await;
    generate_response(ResponseStatus::Success, Some(serde_json::json!(
        {
            "posts": result.0,
            "total": result.1
        }
    )))
}