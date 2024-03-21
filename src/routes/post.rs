use actix_web::{post, Responder, web};
use crate::common::auth::UserData;
use crate::models::post::*;
use crate::common::response::{generate_response, ResponseStatus};
use crate::controller;
use crate::db::PrismaClient;

#[post("/create")]
pub(crate) async fn create_post(client: web::Data<PrismaClient>, body: web::Json<PostCreateReq>, user: UserData) -> impl Responder {
    let result = controller::post::create_post(client, body.title.clone(), &body.content, user.id).await;
    match result {
        Some(post) => {
            generate_response(ResponseStatus::Success, Some(serde_json::json!(
                {
                    "postId": post.id
                }
            )), None)
        }
        None => {
            generate_response(ResponseStatus::BadRequest, None, None)
        }
    }
}