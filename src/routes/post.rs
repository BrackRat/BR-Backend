use actix_web::{get, post, Responder, web};
use crate::prisma::PrismaClient;
use crate::common::auth::UserData;
use crate::common::response::{response};
use crate::operation::pagination::pagination::PaginationReq;
use crate::operation::post::model::{PostCreateReq};
use crate::operation::post::model::Post;

#[post("/create")]
pub async fn create_post(client: web::Data<PrismaClient>, body: web::Json<PostCreateReq>, user: UserData) -> impl Responder {
    let result = Post::create(client, body.into_inner(), user.id).await;
    response(result)
}

#[get("/get")]
pub async fn get_posts(client: web::Data<PrismaClient>, page: web::Query<PaginationReq>) -> impl Responder {
    let result = Post::get_posts(client, page.into_inner()).await;
    response(result)
}