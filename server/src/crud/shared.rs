use rocket::response::Debug;
use rocket::serde::{Deserialize, Serialize};
use rocket_sync_db_pools::{diesel, database};

#[database("diesel")]
pub struct Db(diesel::PgConnection);

pub type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[derive(Deserialize, Serialize, Clone)]
pub struct Contact {
    pub id: i32,
    // person table
    pub firstname: String,
    pub lastname: Option<String>,
    pub nickname: Option<String>,
    pub company: Option<String>,
    pub url: Option<String>,
    pub notes: Option<String>,
    pub favorite: bool,
    pub active: bool,

    // address table
    pub street: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    pub country: Option<String>,

    // email table
    pub emails: Vec<String>,

    // phone number table
    pub phone_numbers: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone, FromForm)]
pub struct ContactForm {
    // person table
    pub firstname: String,
    pub lastname: Option<String>,
    pub nickname: Option<String>,
    pub company: Option<String>,
    pub url: Option<String>,
    pub notes: Option<String>,
    pub favorite: bool,
    pub active: bool,

    // address table
    pub street: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    pub country: Option<String>,

    // email table
    pub emails: Vec<String>,

    // phone number table
    pub phone_numbers: Vec<String>,
}
