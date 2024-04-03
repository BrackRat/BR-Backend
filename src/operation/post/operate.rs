use actix_web::web::Data;
use crate::common::response::ResponseStatus;
use crate::prisma::{post, PrismaClient, user};
use crate::operation::pagination::pagination::{PaginationReq, PaginationRes};
use crate::operation::post::model::{Post, PostCreateReq, PostDetailRes, PostShortRes};
use crate::operation::user::UserShortDetail;

impl Post {
    pub async fn create(prisma: Data<PrismaClient>, input: PostCreateReq, user_id: i32) -> Result<(), ResponseStatus<'static>> {
        let post = prisma
            .post()
            .create(
                input.title,
                vec![
                    post::content::set(input.content),
                    post::SetParam::ConnectUser(user::UniqueWhereParam::IdEquals(user_id)),
                ],
            )
            .exec()
            .await;
        match post {
            Ok(_) => {
                Ok(())
            }
            Err(_) => {
                Err(ResponseStatus::InternalServerError(Some("Cannot create post")))
            }
        }
    }

    pub async fn get_posts(prisma: Data<PrismaClient>, page: PaginationReq) -> Result<PaginationRes<PostShortRes>, ResponseStatus<'static>> {
        let posts = prisma
            .post()
            .find_many(vec![])
            .skip(page.size * (page.page - 1))
            .take(page.size)
            .select(post::select!({
                id
                title
                user: select {
                    id
                    name
                }
            }))
            .exec()
            .await;

        match posts {
            Ok(posts) => {
                let total = prisma
                    .post()
                    .count(vec![])
                    .exec()
                    .await
                    .unwrap();
                let data = posts.into_iter().map(|p| {
                    let user = p.user.unwrap();
                    let author = UserShortDetail {
                        id: user.id,
                        name: user.name,
                    };
                    PostShortRes {
                        cuid: p.id,
                        title: p.title,
                        author,
                    }
                }).collect();

                Ok(PaginationRes {
                    total,
                    data,
                })
            }
            Err(_) => {
                return Err(ResponseStatus::InternalServerError(Some("Cannot get posts")));
            }
        }
    }
}