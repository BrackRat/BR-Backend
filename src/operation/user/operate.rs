use actix_web::web;
use crate::common::response::ResponseStatus;
use crate::db::user;
use crate::operation::operation::{ItemOperation};
use crate::operation::user::model::{User, UserChangePasswordReq, UserGetDetailReq, UserRegisterReq};

impl ItemOperation for User {
    type GetItemsInput = ();
    type GetItemsOutput = Vec<User>;
    type GetDetailInput = UserGetDetailReq;
    type GetDetailOutput = User;
    type ChangeInput = UserChangePasswordReq;
    type ChangeOutput = User;
    type CreateInput = UserRegisterReq;
    type CreateOutput = User;
    type DeleteInput = String;
    type DeleteOutput = User;

    async fn get_items(_: web::Data<crate::db::PrismaClient>,
                       _: Self::GetItemsInput) -> Result<Self::GetItemsOutput, ResponseStatus<'static>> {
        Err(ResponseStatus::InternalServerError(None))
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
            Err(e) => {
                Err(ResponseStatus::InternalServerError(None))
            }
        };
    }
    async fn change(prisma: web::Data<crate::db::PrismaClient>,
                    input: Self::ChangeInput) -> Result<Self::ChangeOutput, ResponseStatus<'static>> {
        Err(ResponseStatus::InternalServerError(None))
    }
    async fn create(prisma: web::Data<crate::db::PrismaClient>,
                    input: Self::CreateInput) -> Result<Self::CreateOutput, ResponseStatus<'static>> {
        Err(ResponseStatus::InternalServerError(None))
    }
    async fn delete(prisma: web::Data<crate::db::PrismaClient>,
                    input: Self::DeleteInput) -> Result<Self::DeleteOutput, ResponseStatus<'static>> {
        Err(ResponseStatus::InternalServerError(None))
    }
}