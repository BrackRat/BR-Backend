use actix_web::{get, post, Responder, web};
use crate::common::jwt::{Claims, verify_jwt};
use crate::common::request;
use crate::common::response::{generate_response, ResponseStatus};
use crate::{controller};
use crate::common::auth::UserData;
use crate::db::PrismaClient;

#[post("/register")]
pub(crate) async fn user_register(client: web::Data<PrismaClient>, body: web::Json<request::UserRegisterReq>) -> impl Responder {
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
pub(crate) async fn user_login(client: web::Data<PrismaClient>, body: web::Json<request::UserLoginReq>) -> impl Responder {
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
            generate_response(ResponseStatus::Unauthorized, None, Some("Username or Password Wrong"))
        }
    }
}

#[get("/me")]
pub(crate) async fn get_user_detail(client: web::Data<PrismaClient>, user: UserData) -> impl Responder {
    let result = controller::user::get_user_detail_from_userid(client, user.id).await;
    match result {
        Some(user) => {
            generate_response(ResponseStatus::Success, Some(serde_json::json!(
                {
                    "user": {
                        "id": user.id,
                        "name": user.name,
                        "registerTime": user.register_unix_timestamp,
                        "lastLoginTime": user.lastlogin_unix_timestamp
                    }
                }
            )), None)
        }
        None => {
            generate_response(ResponseStatus::BadRequest, None, Some("Cannot find user"))
        }
    }
}

#[post("/change_password")]
pub(crate) async fn user_change_password(client: web::Data<PrismaClient>, body: web::Json<request::UserChangePasswordReq>, user: UserData) -> impl Responder {
    let result = controller::user::change_password(client, user.id, body.old_password.clone(), body.new_password.clone()).await;
    match result {
        true => {
            generate_response::<u8>(ResponseStatus::Success, None, None)
        }
        false => {
            generate_response(ResponseStatus::BadRequest, None, None)
        }
    }
}
