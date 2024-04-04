use actix_web::{get, post, put, Responder, web};
use crate::common::response::{response};
use crate::common::auth::UserData;
use crate::prisma::PrismaClient;
use crate::operation::operation::{ItemOperation};
use crate::operation::pagination::pagination::PaginationReq;
use crate::operation::user::model::{UserChangePasswordReq, UserGetDetailReq, UserLoginReq, UserRegisterReq};
use crate::operation::user::{User, UserChangeDetailReq};

#[post("/register")]
pub async fn user_register(client: web::Data<PrismaClient>, body: web::Json<UserRegisterReq>) -> impl Responder {
    let result = User::create(client, body.into_inner()).await;
    response(result)
}

#[post("/login")]
pub async fn user_login(client: web::Data<PrismaClient>, body: web::Json<UserLoginReq>) -> impl Responder {
    let result = User::login(client, body.into_inner()).await;
    response(result)
}

#[get("/me")]
pub async fn get_user_detail(client: web::Data<PrismaClient>, user: UserData) -> impl Responder {
    let user_id = UserGetDetailReq { id: user.id };
    let result = User::get_detail(client, user_id).await;
    response(result)
}

#[post("/change_password")]
pub async fn user_change_password(client: web::Data<PrismaClient>, body: web::Json<UserChangePasswordReq>, user: UserData) -> impl Responder {
    let result = User::change_password(client, user.id, body.into_inner()).await;
    response(result)
}

#[get("/users")]
pub async fn get_users(client: web::Data<PrismaClient>, body: web::Query<PaginationReq>, _user: UserData) -> impl Responder {
    let result = User::get_items(client, body.into_inner()).await;
    response(result)
}

#[put("/me")]
pub async fn change_user_detail(client: web::Data<PrismaClient>, body: web::Json<UserChangeDetailReq>, user: UserData) -> impl Responder {
    let result = User::change_detail(client,  body.into_inner(), user.id).await;
    response(result)
}