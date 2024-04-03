use actix_web::web;
use crate::common::response::ResponseStatus;

pub trait ItemOperation {
    type GetItemsInput;
    type GetItemsOutput;
    type GetDetailInput;
    type GetDetailOutput;
    type ChangeInput;
    type ChangeOutput;
    type CreateInput;
    type CreateOutput;
    type DeleteInput;
    type DeleteOutput;

    async fn get_items(prisma: web::Data<crate::db::PrismaClient>,input: Self::GetItemsInput) -> Result<Self::GetItemsOutput, ResponseStatus<'static>>;
    async fn get_detail(prisma: web::Data<crate::db::PrismaClient>,input: Self::GetDetailInput) -> Result<Self::GetDetailOutput, ResponseStatus<'static>>;
    async fn change(prisma: web::Data<crate::db::PrismaClient>,input: Self::ChangeInput) -> Result<Self::ChangeOutput, ResponseStatus<'static>>;
    async fn create(prisma: web::Data<crate::db::PrismaClient>,input: Self::CreateInput) -> Result<Self::CreateOutput, ResponseStatus<'static>>;
    async fn delete(prisma: web::Data<crate::db::PrismaClient>,input: Self::DeleteInput) -> Result<Self::DeleteOutput, ResponseStatus<'static>>;
}
