use std::io::Cursor;

use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{Responder, Response, Result};
use serde::Serialize;

use super::error_warning::ErrorWarning;

#[derive(Serialize)]
pub struct ResponseWrapper<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<T>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<ErrorWarning>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<ErrorWarning>,
    #[serde(skip_serializing)]
    pub status: Option<Status>,
}

impl<T> ResponseWrapper<T> {
    pub fn new() -> ResponseWrapper<T> {
        ResponseWrapper {
            content: None,
            warnings: vec![],
            errors: vec![],
            status: None,
        }
    }
}

impl<'a, T: Serialize> Responder<'a> for ResponseWrapper<T> {
    fn respond_to(self, _: &Request) -> Result<'a> {
        let str = serde_json::to_string(&self);
        let mut res = Response::build();
        res.header(ContentType::JSON);
        res.sized_body(Cursor::new(str.unwrap()));

        if self.status.is_some() {
            res.status(self.status.unwrap());
        } else if !self.errors.is_empty() {
            res.status(Status::InternalServerError);
        }
        res.ok()
    }
}