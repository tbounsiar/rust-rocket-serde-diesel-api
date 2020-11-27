use crate::config::db::schema::users;

#[table_name = "users"]
#[derive(Queryable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i32>,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}