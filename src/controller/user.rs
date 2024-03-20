use actix_web::{web};
use crate::db::user;
use crate::{utils};

use crate::common::hash_password::*;
use crate::common::jwt::generate_jwt;
use crate::db::user::UniqueWhereParam;


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

pub(crate) async fn login_user(client: web::Data<crate::db::PrismaClient>, name: String, password: String) -> Option<String> {
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
                Some(generate_jwt(user.id, user.name))
            } else {
                None
            }
        }
    };
}

pub(crate) async fn change_password(client: web::Data<crate::db::PrismaClient>, name: String, old_password: String, new_password: String) -> bool {
    let user = client
        .user()
        .find_first(vec![user::name::equals(name.clone())])
        .exec()
        .await
        .unwrap();

    return match user {
        None => {
            false
        }
        Some(user) => {
            if verify_password(old_password.as_str(), user.password.as_str()) {
                let hashed_password = hash_password(&new_password);
                client
                    .user()
                    .update(UniqueWhereParam::IdEquals(user.id), vec![user::password::set(hashed_password.unwrap())])
                    .exec()
                    .await
                    .unwrap();
                return true;
            }
            return false;
        }
    };
}

pub(crate) async fn get_user_detail_from_userid(client: web::Data<crate::db::PrismaClient>, id: i32) -> Option<user::Data> {
    let user = client
        .user()
        .find_unique(user::id::equals(id))
        .exec()
        .await
        .unwrap();
    return user;
}