use diesel::Queryable;
use diesel::Insertable;
use rocket::serde::Serialize;
use crate::db::schema::account;

#[derive(Queryable, Serialize, Clone)]
pub struct Account {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub phone: Option<String>,
    pub password: String
}

#[derive(Insertable, Serialize, Clone)]
#[table_name="account"]
pub struct NewAccountModel {
    pub email: String,
    pub name: String,
    pub phone: Option<String>,
    pub password: String
}

