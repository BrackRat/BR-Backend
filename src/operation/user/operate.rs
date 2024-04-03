use actix_web::web;
use crate::common::hash_password::{hash_password, verify_password};
use crate::common::response::ResponseStatus;
use crate::db::user;
use crate::operation::operation::{ItemOperation};
use crate::operation::pagination::pagination::{PaginationReq, PaginationRes};
use crate::{common, utils};
use crate::db::user::UniqueWhereParam;
use super::model::{TokenRes, User, UserChangePasswordReq, UserGetDetailReq, UserLoginReq, UserRegisterReq, UserShortDetail};

impl ItemOperation for User {
    type GetItemsInput = PaginationReq;
    type GetItemsOutput = PaginationRes<UserShortDetail>;
    type GetDetailInput = UserGetDetailReq;
    type GetDetailOutput = User;
    type ChangeInput = UserChangePasswordReq;
    type ChangeOutput = User;
    type CreateInput = UserRegisterReq;
    type CreateOutput = User;
    type DeleteInput = String;
    type DeleteOutput = User;

    async fn get_items(prisma: web::Data<crate::db::PrismaClient>,
                       page: Self::GetItemsInput) -> Result<Self::GetItemsOutput, ResponseStatus<'static>> {
        let users = prisma
            .user()
            .find_many(vec![])
            .skip(page.size * (page.page - 1))
            .take(page.size)
            .exec()
            .await;

        return match users {
            Ok(users) => {
                let total = prisma
                    .user()
                    .count(vec![])
                    .exec()
                    .await
                    .unwrap();

                let data: Vec<UserShortDetail> = users.into_iter().map(|user| UserShortDetail {
                    id: user.id,
                    name: user.name,
                }).collect();

                Ok(PaginationRes {
                    total,
                    data,
                })
            }

            Err(_) => {
                Err(ResponseStatus::InternalServerError(None))
            }
        };
    }

    async fn get_detail(prisma: web::Data<crate::db::PrismaClient>,
                        input: Self::GetDetailInput) -> Result<Self::GetDetailOutput, ResponseStatus<'static>> {
        let user = prisma
            .user()
            .find_unique(
                user::UniqueWhereParam::IdEquals(input.id)
            )
            .exec()
            .await;
        return match user {
            Ok(user) => {
                let user = user.unwrap();
                Ok(User {
                    name: user.name,
                    last_login_time: user.lastlogin_unix_timestamp.to_string(),
                })
            }
            Err(_) => {
                Err(ResponseStatus::InternalServerError(None))
            }
        };
    }

    async fn change(_: web::Data<crate::db::PrismaClient>,
                    _: Self::ChangeInput) -> Result<Self::ChangeOutput, ResponseStatus<'static>> {
        Err(ResponseStatus::InternalServerError(None))
    }

    async fn create(prisma: web::Data<crate::db::PrismaClient>,
                    input: Self::CreateInput) -> Result<Self::CreateOutput, ResponseStatus<'static>> {
        let is_unique_username = prisma
            .user()
            .find_unique(user::UniqueWhereParam::NameEquals(input.name.clone()))
            .exec()
            .await
            .unwrap();

        if is_unique_username.is_some() {
            return Err(ResponseStatus::BadRequest(Some("Username Already Exists")));
        }

        let hashed_password = hash_password(input.password.as_str());

        return match hashed_password {
            None => {
                Err(ResponseStatus::InternalServerError(Some("Canot Hash Password")))
            }
            Some(hashed_password) => {
                let user = prisma
                    .user()
                    .create(
                        utils::time_utils::get_local_timestamp(),
                        utils::time_utils::get_local_timestamp(),
                        input.name,
                        hashed_password,
                        vec![],
                    )
                    .exec()
                    .await
                    .unwrap();
                Ok(User {
                    name: user.name,
                    last_login_time: user.lastlogin_unix_timestamp.to_string(),
                })
            }
        };
    }

    async fn delete(_: web::Data<crate::db::PrismaClient>,
                    _: Self::DeleteInput) -> Result<Self::DeleteOutput, ResponseStatus<'static>> {
        Err(ResponseStatus::InternalServerError(None))
    }
}

impl User {
    pub async fn login(prisma: web::Data<crate::db::PrismaClient>, input: UserLoginReq) -> Result<TokenRes, ResponseStatus<'static>> {
        let user = prisma
            .user()
            .find_unique(user::UniqueWhereParam::NameEquals(input.name.clone()))
            .exec()
            .await;

        return match user {
            Ok(user) => {
                let user = user.unwrap();
                if verify_password(input.password.as_str(), user.password.as_str()) {
                    Ok(TokenRes {
                        token: common::jwt::generate_jwt(user.id),
                    })
                } else {
                    Err(ResponseStatus::BadRequest(Some("Wrong Password")))
                }
            }
            Err(_) => {
                Err(ResponseStatus::BadRequest(Some("User Not Found")))
            }
        };
    }

    pub async fn change_password(prisma: web::Data<crate::db::PrismaClient>, user_id: i32, req: UserChangePasswordReq) -> Result<(), ResponseStatus<'static>> {
        let user = prisma
            .user()
            .find_first(vec![user::id::equals(user_id)])
            .exec()
            .await
            .unwrap();

        return match user {
            None => {
                Err(ResponseStatus::BadRequest(Some("No Such User")))
            }
            Some(user) => {
                if verify_password(req.old_password.as_str(), user.password.as_str()) {
                    let hashed_password = hash_password(&req.new_password);
                    let r = prisma
                        .user()
                        .update(UniqueWhereParam::IdEquals(user.id), vec![user::password::set(hashed_password.unwrap())])
                        .exec()
                        .await;
                    return if r.is_ok() {
                        Ok(())
                    } else {
                        Err(ResponseStatus::InternalServerError(Some("Cannot change password")))
                    };
                }
                Err(ResponseStatus::BadRequest(Some("No permission")))
            }
        };
    }
}