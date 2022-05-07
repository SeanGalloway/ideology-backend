use crate::db::establish_connection;
use crate::db::model::user_session_model::{NewUserSessionModel, UserSession};
use crate::db::schema::user_session::dsl::*;

use diesel::prelude::*;

pub fn get_session_by_key(_session_key: &uuid::Uuid) -> Option<UserSession> {
    let connection = establish_connection();

    let results = user_session.filter(session_key.eq(_session_key))
        .load::<UserSession>(&connection)
        .expect("Error loading posts");


    if results.get(0).is_some() {
        Some(results.get(0).unwrap().clone())
    } else {
        None
    }
}

pub fn save_session(_session: &NewUserSessionModel) {
    let connection = establish_connection();

    diesel::insert_into(crate::db::schema::user_session::table)
        .values(_session)
        .execute(&connection);
}