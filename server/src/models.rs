use rocket::serde::{Deserialize, Serialize};
use crate::schema::*;

// PEOPLE TABLE
#[derive(Insertable, FromForm, Deserialize, Serialize)]
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
#[derive(Insertable)]
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
#[derive(Insertable)]
#[table_name="emails"]
pub struct Email {
    pub email: String,
}

#[derive(Queryable)]
pub struct EmailEntity {
    pub email_id: String,
    pub email: String,
}

#[derive(Insertable)]
#[table_name="emails_link"]
pub struct EmailLink {
    pub person_id: i32,
    pub email_id: i32,
}

#[derive(Queryable)]
pub struct EmailLinkEntity {
    pub email_link_id: i32,
    pub person_id: i32,
    pub email_id: i32,
}

// PHONE NUMBER TABLE
#[derive(Insertable)]
#[table_name="phone_numbers"]
pub struct PhoneNumber {
    pub num: String,
}

#[derive(Queryable)]
pub struct PhoneNumberEntity {
    pub phone_id: String,
    pub num: String,
}

#[derive(Insertable)]
#[table_name="phone_link"]
pub struct PhoneLink {
    pub person_id: i32,
    pub phone_id: i32,
}

#[derive(Queryable)]
pub struct PhoneLinkEntity {
    pub phone_link_id: i32,
    pub person_id: i32,
    pub phone_id: i32,
}

