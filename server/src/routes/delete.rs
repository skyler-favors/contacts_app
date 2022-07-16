use rocket::fairing::AdHoc;
use rocket::serde::json::Json;

use crate::helper::read::{
    contact_builder, get_address, get_emails, get_people, get_person, get_phones, QueryValue,
};
use crate::helper::shared::{Contact, Db, Result};
use crate::models::PersonEntity;

#[get("/delete/id/<id>")]
async fn delete_by_id(db: Db, id: i32) -> Result<Json<Vec<Contact>>> {
    // return the first person with the given id
    let p: Vec<PersonEntity> = get_person(&db, QueryValue::Id(id)).await?;
    let address = get_address(&db, p[0].address_id).await?;
    let phones = get_phones(&db, p[0].person_id).await?;
    let emails = get_emails(&db, p[0].person_id).await?;
    let contact = vec![contact_builder(p[0].clone(), address, phones, emails)];

    Ok(Json(contact))
}

#[get("/delete/name/<name>")]
async fn delete_by_name(db: Db, name: String) -> Result<Json<Vec<Contact>>> {
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
            .mount("/api", routes![delete_by_id, delete_by_name])
    })
}
