use chrono::NaiveDateTime;
use diesel::prelude::*;
use crate::schema::users;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Serialize)]
pub struct GenericResponse<T> {
    pub status: String,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Debug, Serialize, Deserialize, Insertable, Queryable)]
#[diesel(table_name = users)]
pub struct Users {
    pub id: Option<i32>,
    pub user_id: Uuid, 
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct User {
    pub id: i32,
    pub user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(AsChangeset, Debug, Deserialize)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
}