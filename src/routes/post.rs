use actix_web::{get, post, delete, put, Responder, web};
use actix_web::web::Data;
use crate::prisma::PrismaClient;
use crate::common::auth::UserData;
use crate::common::response::{response};
use crate::operation::pagination::pagination::PaginationReq;
use crate::operation::post::model::{PostCreateReq, PostEditReq};
use crate::operation::post::model::Post;

#[post("")]
pub async fn create_post(client: web::Data<PrismaClient>, body: web::Json<PostCreateReq>, user: UserData) -> impl Responder {
    let result = Post::create(client, body.into_inner(), user.id).await;
    response(result)
}

#[get("/many")]
pub async fn get_posts(client: web::Data<PrismaClient>, page: web::Query<PaginationReq>) -> impl Responder {
    let result = Post::get_posts(client, page.into_inner()).await;
    response(result)
}

#[get("/{post_id}")]
pub async fn get_post_detail(client: web::Data<PrismaClient>, redis_client: Data<redis::Client>, post_id: web::Path<String>) -> impl Responder {
    let result = Post::get(client, redis_client, post_id.into_inner()).await;
    response(result)
}

#[delete("/{post_id}")]
pub async fn delete_post(client: web::Data<PrismaClient>, post_id: web::Path<String>, user: UserData) -> impl Responder {
    let result = Post::delete(client, post_id.into_inner(), user.id).await;
    response(result)
}

#[put("/{post_id}")]
pub async fn edit_post(client: web::Data<PrismaClient>, post_id: web::Path<String>, body: web::Json<PostEditReq>, user: UserData) -> impl Responder {
    let result = Post::put(client, post_id.into_inner(), body.into_inner(), user.id).await;
    response(result)
}