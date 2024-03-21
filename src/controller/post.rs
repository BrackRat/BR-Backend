use actix_web::web;
use crate::db::{post};

pub(crate) async fn create_post(client: web::Data<crate::db::PrismaClient>, title: String, content: &Option<String>, user_id: i32) -> Option<post::Data> {
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