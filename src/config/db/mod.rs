use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;

use crate::common::result::{ApiResult, Error, ErrorType};

pub mod schema;

pub fn connection() -> ApiResult<PgConnection> {
    dotenv().ok();
    match env::var("DATABASE_URL") {
        Ok(url) => {
            match PgConnection::establish(url.as_str()) {
                Ok(connection) => Ok(connection),
                Err(e) => Err(
                    Error::new(ErrorType::DbConnection, Some(e.to_string()))
                ),
            }
        }
        Err(e) => Err(Error::new(ErrorType::EnvError, Some(format!("DATABASE_URL {}", e.to_string()))))
    }
}