use rocket::http::Status;

use crate::api::dao::client_dao;
use crate::api::model::Client;
use crate::common::http::{ErrorWarning, ResponseWrapper};

pub fn list() -> ResponseWrapper<Vec<Client>> {
    let mut response: ResponseWrapper<Vec<Client>> = ResponseWrapper::new();
    match client_dao::list() {
        Ok(c) => response.content = Some(c),
        Err(e) => {
            response.errors.push(ErrorWarning { code: 0, message: e.to_string() });
            response.status = Some(Status::NotFound);
        }
    }
    response
}

pub fn get(id: i32) -> ResponseWrapper<Client> {
    let mut response: ResponseWrapper<Client> = ResponseWrapper::new();
    match client_dao::get(id) {
        Ok(c) => response.content = Some(c),
        Err(e) => {
            response.errors.push(ErrorWarning { code: 0, message: e.to_string() });
            response.status = Some(Status::NotFound);
        }
    }
    response
}

pub fn create(client: Client) -> ResponseWrapper<Client> {
    let mut response: ResponseWrapper<Client> = ResponseWrapper::new();
    match client_dao::create(client) {
        Ok(c) => response.content = Some(c),
        Err(e) => {
            response.errors.push(ErrorWarning { code: 0, message: e.to_string() });
            response.status = Some(Status::NotFound);
        }
    }
    response
}

pub fn update(client: Client) -> ResponseWrapper<Client> {
    let mut response: ResponseWrapper<Client> = ResponseWrapper::new();
    match client_dao::update(client) {
        Ok(c) => response.content = Some(c),
        Err(e) => {
            response.errors.push(ErrorWarning { code: 0, message: e.to_string() });
            response.status = Some(Status::NotFound);
        }
    }
    response
}

pub fn delete(id: i32) -> ResponseWrapper<bool> {
    let mut response: ResponseWrapper<bool> = ResponseWrapper::new();
    match client_dao::delete(id) {
        Ok(c) => response.content = Some(c),
        Err(e) => {
            response.errors.push(ErrorWarning { code: 0, message: e.to_string() });
            response.status = Some(Status::NotFound);
        }
    }
    response
}
