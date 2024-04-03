use actix_web::web;
use crate::prisma::{post};
use crate::prisma::post::Data;

pub async fn create_post(client: web::Data<crate::prisma::PrismaClient>, title: String, content: &Option<String>, user_id: i32) -> Option<post::Data> {
    let post = client
        .post()
        .create(
            title,
            vec![post::content::set(content.clone()), post::SetParam::SetUserId(Some(user_id))],
        )
        .exec()
        .await
        .unwrap();
    Some(post)
}

pub async fn get_posts(client: web::Data<crate::prisma::PrismaClient>, page: i64, size: i64) -> (Vec<Data>, i64) {
    let skip = (page - 1) * size;
    let total = client
        .post()
        .count(vec![])
        .exec()
        .await
        .unwrap();

    let posts = client
        .post()
        .find_many(vec![])
        .skip(skip)
        .take(size)
        .exec()
        .await
        .unwrap();
    (posts, total)
}