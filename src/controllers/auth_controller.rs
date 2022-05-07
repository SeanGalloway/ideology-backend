use rocket::serde::json::Json;
use rocket::response::status;
use rocket::response::status::Custom;
use crate::controllers::dto::{Login, NewAccount};
use crate::controllers::validation;
use crate::controllers::input_processing as ip;
use crate::db::account_repository as db;
use crate::db::user_session_repository as session_db;
use crate::db::model::account_model::{Account, NewAccountModel};
use uuid::Uuid;
use crate::db::model::user_session_model::{NewUserSessionModel, UserSession};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};


#[post("/auth/register", data="<account>")]
pub fn register(account: Json<NewAccount<'_>>) -> status::Custom<&str> {
    let _account = account.0;

    let phone_number: Option<String> = if _account.phone.is_some() { Some(ip::clean_phone_number(String::from(_account.phone.unwrap())))} else { Option::None };

    let phone_number_is_valid = if phone_number.is_some() {
        validation::phone_number_is_valid(&phone_number.as_ref().unwrap())
    } else {
        true
    };

    if validation::password_is_valid(_account.password) && phone_number_is_valid {
        let account_model = NewAccountModel {
            email: String::from(_account.email),
            name: String::from(_account.name),
            phone: phone_number,
            password: ip::hash_password(String::from(_account.password))
        };

        db::save_account(&account_model);

        status::Custom(rocket::http::Status::Accepted, "Successfully registered")
    } else {
        status::Custom(rocket::http::Status::BadRequest, "Error with registration")
    }
}


#[post("/auth/login", data="<login>")]
pub fn login(login: Json<Login<'_>>) -> Custom<String> {
    let account = db::get_account_by_email(login.0.email);

    if account.is_some() {
        if ip::verify_password(login.0.password, &account.as_ref().unwrap().password) {
            let uuid = Uuid::new_v4();

            let session = NewUserSessionModel {
                session_key: uuid.clone(),
                user_id: account.unwrap().id,
                is_valid: true,
            };

            session_db::save_session(&session);

            let token = encode(&Header::default(), &session, &EncodingKey::from_secret("secret".as_ref())).unwrap();

            return status::Custom(rocket::http::Status::Accepted, token)
        }
    }

    return status::Custom(rocket::http::Status::Unauthorized, String::from("Access Denied"))
}