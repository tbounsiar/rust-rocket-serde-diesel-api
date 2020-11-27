use crate::api::dao::client_dao;
use crate::api::model::Client;

pub fn list() -> Vec<Client> {
    client_dao::list()
}

pub fn get(id: i32) -> Client {
    client_dao::get(id)
}

pub fn create(client: Client) -> Client {
    client_dao::create(client)
}

pub fn update(client: Client) -> Client {
    client_dao::update(client)
}

pub fn delete(id: i32) -> bool {
    client_dao::delete(id)
}
