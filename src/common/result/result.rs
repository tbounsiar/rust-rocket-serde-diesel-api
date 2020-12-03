#[derive(Debug, PartialEq)]
pub enum ErrorType {
    DbConnection,
    DbError,
    EnvError,
}

#[derive(Debug, PartialEq)]
pub struct Error {
    pub error_type: ErrorType,
    pub message: Option<String>,
}

impl Error {
    pub fn new(error_type: ErrorType, message: Option<String>) -> Error {
        Error { error_type, message }
    }
}

impl std::string::ToString for Error {
    fn to_string(&self) -> String {
        String::from(self.message.as_ref().unwrap())
    }
}

pub type ApiResult<T> = Result<T, Error>;