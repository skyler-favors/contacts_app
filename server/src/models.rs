use rocket::serde::{Deserialize, Serialize};
use crate::schema::*;

// PEOPLE TABLE
#[derive(Insertable, Deserialize, Serialize, AsChangeset)]
#[table_name = "people"]
pub struct Person {
    pub firstname: String,
    pub lastname: String,
    pub nickname: String,
    pub company: String,
    pub url: String,
    pub notes: String,
    pub favorite: bool,
    pub active: bool,
    pub address_id: i32,
}

#[derive(Queryable, Serialize, Clone)]
pub struct PersonEntity {
    pub person_id: i32,
    pub firstname: String,
    pub lastname: String,
    pub nickname: String,
    pub company: String,
    pub url: String,
    pub notes: String,
    pub favorite: bool,
    pub active: bool,
    pub address_id: i32,
}

// ADDRESS TABLE
#[derive(Insertable, AsChangeset)]
#[table_name="addresses"]
pub struct Address {
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub country: String,
}

#[derive(Queryable)]
pub struct AddressEntity {
    pub address_id: i32,
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub country: String,
}

// EMAIL TABLE
#[derive(Insertable, AsChangeset)]
#[table_name="emails"]
pub struct Email {
    pub person_id: i32,
    pub email: String,
}

#[derive(Queryable)]
pub struct EmailEntity {
    pub email_id: i32,
    pub person_id: i32,
    pub email: String,
}

// PHONE NUMBER TABLE
#[derive(Insertable, AsChangeset)]
#[table_name="phone_numbers"]
pub struct PhoneNumber {
    pub person_id: i32,
    pub num: String,
}

#[derive(Queryable)]
pub struct PhoneNumberEntity {
    pub phone_id: i32,
    pub person_id: i32,
    pub num: String,
}
