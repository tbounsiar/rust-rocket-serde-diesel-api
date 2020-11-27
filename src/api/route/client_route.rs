use rocket::Route;
use rocket_contrib::json::{Json, JsonValue};

use crate::api::model::Client;
use crate::api::service::client_service;

#[get("/")]
fn list() -> Json<Vec<Client>> {
    Json(client_service::list())
}

#[get("/<id>")]
fn get(id: i32) -> Json<Client> {
    Json(client_service::get(id))
}

#[post("/", data = "<client>")]
fn create(client: Json<Client>) -> Json<Client> {
    Json(client_service::create(client.into_inner()))
}

#[put("/", data = "<client>")]
fn update(client: Json<Client>) -> Json<Client> {
    Json(client_service::update(client.into_inner()))
}

#[delete("/<id>")]
fn delete(id: i32) -> Json<JsonValue> {
    let success = client_service::delete(id);
    Json(json!({
        "success": success
    }))
}

pub fn routes() -> Vec<Route> {
    routes![list, get, create, update, delete]
}