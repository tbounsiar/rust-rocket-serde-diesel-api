extern crate rocket;

use rocket::Rocket;

pub mod model;
pub mod dao;
pub mod service;
pub mod route;

pub fn init() -> Rocket {
    rocket::ignite()
        .mount("/client", route::client_route::routes())
        .mount("/user", route::user_route::routes())
}