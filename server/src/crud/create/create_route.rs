use rocket::serde::json::Json;
use rocket::fairing::AdHoc;
use rocket::response::status::Created;

use crate::crud::shared::{Contact, Result, Db};
use super::create::{insert_person, insert_addresses, insert_emails, insert_phone_numbers};

#[post("/create", data = "<contact>")]
async fn create(db: Db, contact: Json<Contact>) -> Result<Created<Json<Contact>>> {
    let address_id = insert_addresses(&db, &contact).await?;
    let person_id = insert_person(&db, &contact, address_id).await?;

    insert_emails(&db, &contact, person_id).await?;
    insert_phone_numbers(&db, &contact, person_id).await?;

     Ok(Created::new("/create").body(contact))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel Stage", |rocket| async {
        rocket
            .mount("/api", routes![create])
    })
}
