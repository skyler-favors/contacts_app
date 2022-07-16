use rocket::fairing::AdHoc;
use rocket::serde::json::Json;

use crate::helper::read::{
    contact_builder, get_address, get_emails, get_people, get_person, get_phones, QueryValue,
};
use crate::helper::shared::{Contact, Db, Result};
use crate::models::PersonEntity;

use super::read::{query_by_name, query_by_id};

#[post("/update/id/<id>", data = "<new_contact>")]
async fn update_by_id(db: Db, id: i32, new_contact: Json<Contact>) -> Result<Json<Vec<Contact>>> {
    let contacts = query_by_id(db, id).await?;

    Ok(contacts)
}

#[post("/update/name/<name>", data = "<contact>")]
async fn update_by_name(db: Db, name: String, contact: Json<Contact>) -> Result<Json<Vec<Contact>>> {
    // return first person found with the given name
    // uses LIKE for fuzzy matching
    let people: Vec<PersonEntity> = get_person(&db, QueryValue::Name(name)).await?;

    let mut contacts: Vec<Contact> = Vec::new();
    for p in people {
        let address = get_address(&db, p.address_id).await?;
        let phones = get_phones(&db, p.person_id).await?;
        let emails = get_emails(&db, p.person_id).await?;
        let contact = contact_builder(p, address, phones, emails);
        contacts.push(contact);
    }

    Ok(Json(contacts))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel Stage", |rocket| async {
        rocket
            .attach(Db::fairing())
            .mount("/api", routes![update_by_id, update_by_name])
    })
}
