use crate::api::dao::user_dao;
use crate::api::model::User;

pub fn list() -> Vec<User> {
    user_dao::list()
}

pub fn get(id: i32) -> User {
    user_dao::get(id)
}

pub fn create(user: User) -> User {
    user_dao::create(user)
}

pub fn update(user: User) -> User {
    user_dao::update(user)
}

pub fn delete(id: i32) -> bool {
    user_dao::delete(id)
}
