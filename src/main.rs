#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

extern crate argon2;

extern crate uuid;

extern crate jsonwebtoken;

mod controllers;
mod cors;
mod db;

use cors::Cors;

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {

    let cors = Cors {};

    rocket::build()
        .mount("/", routes![
            controllers::text_controller::get_text,
            controllers::auth_controller::login,
            controllers::auth_controller::register
        ])
        .attach(cors)
}