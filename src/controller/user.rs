use actix_web::{web};
use crate::db::user;
use crate::{utils};

pub(crate) async fn register_user(client: web::Data<crate::db::PrismaClient>, name: String, password: String) -> Option<user::Data> {
    let is_unique_username = client
        .user()
        .find_first(vec![user::name::equals(name.clone())])
        .exec()
        .await
        .unwrap();

    if is_unique_username.is_some() {
        return None;
    }

    // ToDo Password hash

    let user = client
        .user()
        .create(
            utils::time_utils::get_local_timestamp(),
            name.clone(),
            password.clone(),
            vec![],
        )
        .exec()
        .await
        .unwrap();
    return Some(user);
}