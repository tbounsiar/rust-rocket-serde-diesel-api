extern crate diesel;

use diesel::prelude::*;

use crate::api::model::User;
use crate::config::db::connection;
use crate::config::db::schema::users::dsl::*;

pub fn list() -> Vec<User> {
    let connection = connection();
    users
        .order(id.desc())
        .load::<User>(&connection)
        .expect("error listing users")
}

pub fn get(ID: i32) -> User {
    let connection = connection();
    let results = users
        .find(ID)
        .first::<User>(&connection)
        .expect("error listing users");
    results
}

pub fn create(user: User) -> User {
    let connection = connection();
    let result = diesel::insert_into(users)
        .values(&user)
        .get_result(&connection)
        .expect("Error creating new User");
    result
}

pub fn update(user: User) -> User {
    let connection = connection();
    diesel::update(users.find(user.id))
        .set(&user)
        .execute(&connection)
        .expect("Error creating new User");
    user
}

pub fn delete(ID: i32) -> bool {
    let connection = connection();
    diesel::delete(users.find(ID))
        .execute(&connection)
        .is_ok()
}