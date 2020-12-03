extern crate diesel;

use diesel::prelude::*;

use crate::api::model::Client;
use crate::common::result::{ApiResult, Error, ErrorType};
use crate::config::db::connection;
use crate::config::db::schema::clients::dsl::*;

pub fn list() -> ApiResult<Vec<Client>> {
    match connection() {
        Ok(connection) => {
            match clients.order(id.desc()).load::<Client>(&connection) {
                Ok(c) => Ok(c),
                Err(e) => Err(Error::new(ErrorType::DbError, Some(e.to_string()))),
            }
        }
        Err(mut e) => {
            println!("{}", e.message.unwrap());
            e.message = Some(String::from("Db Connection ERROR"));
            Err(e)
        }
    }
}

pub fn get(ID: i32) -> ApiResult<Client> {
    match connection() {
        Ok(connection) => {
            match clients.find(ID).first::<Client>(&connection) {
                Ok(c) => Ok(c),
                Err(e) => Err(Error::new(ErrorType::DbError, Some(e.to_string()))),
            }
        }
        Err(e) => Err(e)
    }
}

pub fn create(client: Client) -> ApiResult<Client> {
    match connection() {
        Ok(connection) => {
            let result = diesel::insert_into(clients).
                values(&client).
                get_result::<Client>(&connection);
            match result {
                Ok(c) => Ok(c),
                Err(e) => Err(Error::new(ErrorType::DbError, Some(e.to_string()))),
            }
        }
        Err(e) => Err(e)
    }
}

pub fn update(client: Client) -> ApiResult<Client> {
    match connection() {
        Ok(connection) => {
            let result = diesel::update(clients.find(client.id))
                .set(&client)
                .execute(&connection);
            match result {
                Ok(c) => Ok(client),
                Err(e) => Err(Error::new(ErrorType::DbError, Some(e.to_string()))),
            }
        }
        Err(e) => Err(e)
    }
}

pub fn delete(ID: i32) -> ApiResult<bool> {
    match connection() {
        Ok(connection) => {
            match diesel::delete(clients.find(ID))
                .execute(&connection) {
                Ok(c) => Ok(true),
                Err(e) => Err(Error::new(ErrorType::DbError, Some(e.to_string()))),
            }
        }
        Err(e) => Err(e)
    }
}