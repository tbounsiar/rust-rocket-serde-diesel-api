extern crate diesel;

use diesel::prelude::*;

use crate::api::model::Client;
use crate::config::db::connection;
use crate::config::db::schema::clients::dsl::*;

pub fn list() -> Vec<Client> {
    let connection = connection();
    clients
        .order(id.desc())
        .load::<Client>(&connection)
        .expect("error listing clients")
}

pub fn get(ID: i32) -> Client {
    let connection = connection();
    let results = clients
        .find(ID)
        .first::<Client>(&connection)
        .expect("error listing clients");
    results
}

pub fn create(client: Client) -> Client {
    let connection = connection();
    let result = diesel::insert_into(clients)
        .values(&client)
        .get_result(&connection)
        .expect("Error creating new Client");
    result
}

pub fn update(client: Client) -> Client {
    let connection = connection();
    diesel::update(clients.find(client.id))
        .set(&client)
        .execute(&connection)
        .expect("Error creating new Client");
    client
}

pub fn delete(ID: i32) -> bool {
    let connection = connection();
    diesel::delete(clients.find(ID))
        .execute(&connection)
        .is_ok()
}