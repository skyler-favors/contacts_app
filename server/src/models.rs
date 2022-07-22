use rocket::serde::{Deserialize, Serialize};
use crate::schema::*;

// PEOPLE TABLE
#[derive(Insertable, Deserialize, Serialize, AsChangeset)]
#[table_name = "people"]
pub struct Person {
    pub firstname: String,
    pub lastname: Option<String>,
    pub nickname: Option<String>,
    pub company: Option<String>,
    pub url: Option<String>,
    pub notes: Option<String>,
    pub favorite: bool,
    pub active: bool,
    pub address_id: i32,
}

#[derive(Queryable, Serialize, Clone)]
pub struct PersonEntity {
    pub person_id: i32,
    pub firstname: String,
    pub lastname: Option<String>,
    pub nickname: Option<String>,
    pub company: Option<String>,
    pub url: Option<String>,
    pub notes: Option<String>,
    pub favorite: bool,
    pub active: bool,
    pub address_id: i32,
}

// ADDRESS TABLE
#[derive(Insertable, AsChangeset)]
#[table_name="addresses"]
pub struct Address {
    pub street: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    pub country: Option<String>,
}

#[derive(Queryable)]
pub struct AddressEntity {
    pub address_id: i32,
    pub street: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    pub country: Option<String>,
}

// EMAIL TABLE
#[derive(Insertable, Deserialize, Serialize, AsChangeset, Clone)]
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
