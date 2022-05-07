use rocket::serde::{Deserialize, json::Json};

#[derive(Deserialize)]
pub struct NewAccount<'r> {
    pub email: &'r str,
    pub name: &'r str,
    pub phone: Option<&'r str>,
    pub password: &'r str
}

#[derive(Deserialize)]
pub struct Login<'r> {
    pub email: &'r str,
    pub password: &'r str
}


