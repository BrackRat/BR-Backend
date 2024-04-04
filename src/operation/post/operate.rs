use actix_web::web::Data;
use crate::{generate_update_params};
use crate::common::response::ResponseStatus;
use crate::prisma::{post, PrismaClient, user};
use crate::operation::pagination::pagination::{PaginationReq, PaginationRes};
use crate::operation::post::model::{Post, PostCreateReq, PostCreateRes, PostDetailRes, PostEditReq, PostShortRes};
use crate::operation::user::UserShortDetail;

impl Post {
    pub async fn create(prisma: Data<PrismaClient>, input: PostCreateReq, user_id: i32) -> Result<PostCreateRes, ResponseStatus<'static>> {
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
            Ok(post) => {
                Ok(PostCreateRes {
                    cuid: post.id
                })
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

    pub async fn get(prisma: Data<PrismaClient>, id: String) -> Result<PostDetailRes, ResponseStatus<'static>> {
        let post = prisma
            .post()
            .find_unique(post::UniqueWhereParam::IdEquals(id))
            .select(
                post::select!({
                    id
                    title
                    content
                    user: select {
                        id
                        name
                    }
                })
            )
            .exec()
            .await;

        match post {
            Ok(post) => {
                if post.is_none() {
                    return Err(ResponseStatus::NotFound(Some("Post not found")));
                }
                let post = post.unwrap();
                let user = post.user.unwrap();
                let author = UserShortDetail {
                    id: user.id,
                    name: user.name,
                };
                Ok(PostDetailRes {
                    title: post.title,
                    content: post.content,
                    author,
                })
            }
            Err(_) => {
                return Err(ResponseStatus::InternalServerError(Some("Cannot get post")));
            }
        }
    }

    async fn post_own_user(prisma: Data<PrismaClient>, id: String, user_id: i32) -> bool {
        let post = prisma
            .post()
            .find_unique(post::UniqueWhereParam::IdEquals(id.clone()))
            .select(
                post::select!({
                    user: select {
                        id
                    }
                })
            )
            .exec()
            .await;

        match post {
            Ok(post) => {
                let post = post.unwrap();
                if post.user.unwrap().id == user_id {
                    return true;
                }
                false
            }
            Err(_) => {
                false
            }
        }
    }

    pub async fn delete(prisma: Data<PrismaClient>, id: String, user_id: i32) -> Result<(), ResponseStatus<'static>> {
        if !Self::post_own_user(prisma.clone(), id.clone(), user_id).await {
            return Err(ResponseStatus::BadRequest(Some("You are not the author of the post")));
        }

        let post = prisma
            .post()
            .delete(post::UniqueWhereParam::IdEquals(id))
            .exec()
            .await;
        match post {
            Ok(_) => {
                Ok(())
            }
            Err(_) => {
                Err(ResponseStatus::InternalServerError(Some("Cannot delete post")))
            }
        }
    }


    pub async fn put(prisma: Data<PrismaClient>, id: String, input: PostEditReq, user_id: i32) -> Result<(), ResponseStatus<'static>> {
        if !Self::post_own_user(prisma.clone(), id.clone(), user_id).await {
            return Err(ResponseStatus::BadRequest(Some("You are not the author of the post")));
        }

        let mut update_params = vec![];

        generate_update_params!(post, update_params;
            SetTitle: input.title,
            SetContent: Some(input.content),
        );

        let post = prisma
            .post()
            .update(
                post::UniqueWhereParam::IdEquals(id),
                update_params,
            )
            .exec()
            .await;
        match post {
            Ok(_) => {
                Ok(())
            }
            Err(_) => {
                Err(ResponseStatus::InternalServerError(Some("Cannot update post")))
            }
        }
    }
}


