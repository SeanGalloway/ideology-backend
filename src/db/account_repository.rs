use crate::db::establish_connection;
use crate::db::model::account_model::{Account, NewAccountModel};
use crate::db::schema::account::dsl::*;
use diesel::prelude::*;
use core::option::Option;

pub fn get_account_by_email(_email: &str) -> Option<Account> {
    let connection = establish_connection();

    let results = account.filter(email.eq(_email))
        .load::<Account>(&connection)
        .expect("Error loading posts");

    if results.get(0).is_some() {
        Some(results.get(0).unwrap().clone())
    } else {
        None
    }
}

pub fn save_account(_new_account: &NewAccountModel) {
    let connection = establish_connection();

    diesel::insert_into(crate::db::schema::account::table)
        .values(_new_account)
        .execute(&connection);
}