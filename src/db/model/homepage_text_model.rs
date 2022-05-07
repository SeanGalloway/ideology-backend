use diesel::Queryable;
use rocket::serde::Serialize;

#[derive(Queryable, Serialize, Clone)]
pub struct HomepageText {
    pub id: i32,
    pub title: String,
    pub body: String
}