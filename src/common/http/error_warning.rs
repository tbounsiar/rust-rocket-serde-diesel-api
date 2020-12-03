#[derive(Serialize)]
pub struct ErrorWarning {
    pub code: i32,
    pub message: String,
}