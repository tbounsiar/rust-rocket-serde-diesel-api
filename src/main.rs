#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use rocket_contrib::serve::StaticFiles;

mod api;
mod config;

fn main() {
    api::init()
        .mount("/", StaticFiles::from("web"))
        .launch();
}
