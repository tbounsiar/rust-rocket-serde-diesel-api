use rocket::Route;
use rocket_contrib::json::{Json, JsonValue};

use crate::api::model::User;
use crate::api::service::user_service;

#[get("/")]
fn list() -> Json<Vec<User>> {
    Json(user_service::list())
}

#[get("/<id>")]
fn get(id: i32) -> Json<User> {
    Json(user_service::get(id))
}

#[post("/", data = "<user>")]
fn create(user: Json<User>) -> Json<User> {
    Json(user_service::create(user.into_inner()))
}

#[put("/", data = "<user>")]
fn update(user: Json<User>) -> Json<User> {
    Json(user_service::update(user.into_inner()))
}

#[delete("/<id>")]
fn delete(id: i32) -> Json<JsonValue> {
    let success = user_service::delete(id);
    Json(json!({
        "success": success
    }))
}

pub fn routes() -> Vec<Route> {
    routes![list, get, create, update, delete]
}