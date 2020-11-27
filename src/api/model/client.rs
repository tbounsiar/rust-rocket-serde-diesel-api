use crate::config::db::schema::clients;

#[table_name = "clients"]
#[derive(Queryable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
pub struct Client {
    pub id: Option<i32>,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}