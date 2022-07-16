use rocket::response::Debug;
use rocket_sync_db_pools::diesel;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;

use crate::models::{PersonEntity, AddressEntity, Person, Address, Email, PhoneNumber};

#[database("diesel")]
pub struct Db(diesel::PgConnection);
pub type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[derive(Deserialize, Serialize, Clone)]
pub struct Contact {
    // person table
    pub firstname: String,
    pub lastname: String,
    pub nickname: String,
    pub company: String,
    pub url: String,
    pub notes: String,
    pub favorite: bool,
    pub active: bool,

    // address table
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub country: String,

    // email table
    pub emails: Vec<String>,

    // phone number table
    pub phone_numbers: Vec<String>,
}
