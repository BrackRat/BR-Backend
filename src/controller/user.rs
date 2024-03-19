use actix_web::{web};
use crate::db::user;
use crate::{utils};

use crate::common::hash_password::*;
use crate::db::user::Data;


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
    let hashed_password = hash_password(&password);

    return match hashed_password {
        None => {
            None
        }
        Some(hashed_password) => {
            let user = client
                .user()
                .create(
                    utils::time_utils::get_local_timestamp(),
                    name,
                    hashed_password,
                    vec![],
                )
                .exec()
                .await
                .unwrap();
            Some(user)
        }
    };
}

pub(crate) async fn login_user(client: web::Data<crate::db::PrismaClient>, name: String, password: String) -> Option<user::Data> {
    let user = client
        .user()
        .find_first(vec![user::name::equals(name.clone())])
        .exec()
        .await
        .unwrap();

    return match user {
        None => {
            None
        }
        Some(user) => {
            if verify_password(password.as_str(), user.password.as_str()) {
                Some(user)
            } else {
                None
            }
        }
    }
}