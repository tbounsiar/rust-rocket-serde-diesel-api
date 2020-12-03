use rocket::Route;
use rocket_contrib::json::Json;

use crate::api::model::Client;
use crate::api::service::client_service;
use crate::common::http::ResponseWrapper;

#[get("/")]
fn list() -> ResponseWrapper<Vec<Client>> {
    client_service::list()
}

#[get("/<id>")]
fn get(id: i32) -> ResponseWrapper<Client> {
    client_service::get(id)
}

#[post("/", data = "<client>")]
fn create(client: Json<Client>) -> ResponseWrapper<Client> {
    client_service::create(client.into_inner())
}

#[put("/", data = "<client>")]
fn update(client: Json<Client>) -> ResponseWrapper<Client> {
    client_service::update(client.into_inner())
}

#[delete("/<id>")]
fn delete(id: i32) -> ResponseWrapper<bool> {
    client_service::delete(id)
}

pub fn routes() -> Vec<Route> {
    routes![list, get, create, update, delete]
}