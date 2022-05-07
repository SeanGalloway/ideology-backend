extern crate diesel;
extern crate rocket;


use crate::db::homepage_text_repository;
use rocket::serde::{Deserialize, json::Json};


#[get("/text/<key>")]
pub fn get_text(key: &str) -> Option<String> {
    return homepage_text_repository::get_text_by_key(key)
}

