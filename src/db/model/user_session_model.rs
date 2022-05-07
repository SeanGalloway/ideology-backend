use diesel::expression::AsExpression;
use diesel::Queryable;
use diesel::Insertable;
use rocket::serde::Serialize;
use crate::db::schema::user_session;
use std::time::SystemTime;


#[derive(Queryable, Insertable, Clone, Serialize)]
#[table_name="user_session"]
pub struct UserSession {
    pub session_key: uuid::Uuid,
    pub user_id: i32,
    pub is_valid: bool,
    pub creation_timestamp: SystemTime
}

#[derive(Insertable, Clone, Serialize)]
#[table_name="user_session"]
pub struct NewUserSessionModel {
    pub session_key: uuid::Uuid,
    pub user_id: i32,
    pub is_valid: bool
}
