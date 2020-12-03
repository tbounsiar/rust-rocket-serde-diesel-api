extern crate diesel;

use diesel::prelude::*;

use crate::api::model::User;
use crate::config::db::connection;
use crate::config::db::schema::users::dsl::*;

pub fn list() -> Vec<User> {
    let connection = connection().unwrap();
    users
        .order(id.desc())
        .load::<User>(&connection)
        .expect("result listing users")
}

pub fn get(ID: i32) -> User {
    let connection = connection().unwrap();
    let results = users
        .find(ID)
        .first::<User>(&connection)
        .expect("result listing users");
    results
}

pub fn create(user: User) -> User {
    let connection = connection().unwrap();
    let result = diesel::insert_into(users)
        .values(&user)
        .get_result(&connection)
        .expect("Error creating new User");
    result
}

pub fn update(user: User) -> User {
    let connection = connection().unwrap();
    diesel::update(users.find(user.id))
        .set(&user)
        .execute(&connection)
        .expect("Error creating new User");
    user
}

pub fn delete(ID: i32) -> bool {
    let connection = connection().unwrap();
    diesel::delete(users.find(ID))
        .execute(&connection)
        .is_ok()
}