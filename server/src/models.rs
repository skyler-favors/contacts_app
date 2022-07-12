use rocket::serde::{Deserialize, Serialize};

use crate::schema::*;

#[derive(Insertable, FromForm, Deserialize, Serialize)]
#[table_name = "users"]
pub struct User {
    pub name: String,
    pub email: String,
    pub age: i32,
}

#[derive(Queryable, Debug, Serialize)]
pub struct UserEntity {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub age: i32,
}

#[derive(Insertable)]
#[table_name="contacts"]
pub struct Contact {
    pub user_id: i32,
    pub name: String,
    pub phone_number: String,
}

#[derive(Queryable)]
pub struct ContactEntity {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub phone_number: String,
}

#[derive(Insertable)]
pub struct AuthInfo {
    pub user_id: i32,
    pub password_hash: String,
}

#[derive(Queryable)]
pub struct AuthInfoEntity {
    pub id: i32,
    pub user_id: i32,
    pub password_hash: String,
}
