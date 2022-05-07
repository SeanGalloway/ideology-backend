use crate::db::establish_connection;
use crate::db::model::homepage_text_model::HomepageText;
use crate::db::schema::homepage_text::dsl::*;

use diesel::prelude::*;
use core::option::Option;

pub fn get_text_by_key(key: &str) -> Option<String> {

    let connection = establish_connection();

    let results = homepage_text.filter(title.eq(key))
        .load::<HomepageText>(&connection)
        .expect("Error loading posts");


    return if results.get(0).is_some() {
         Some(results.get(0).unwrap().body.clone())
    } else {
        None
    }
}