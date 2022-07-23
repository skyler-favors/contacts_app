use rocket::fairing::AdHoc;
use rocket::serde::json::Json;

use super::read_helper::{
    contact_builder, get_address, get_emails, get_people, get_people_by_name, get_people_in_trash,
    get_person_by_id, get_phones,
};
use crate::crud::shared::{Contact, Db, Result};
use crate::models::PersonEntity;

#[get("/read/all")]
async fn list(db: Db) -> Result<Json<Vec<Contact>>> {
    // select all from people table
    let people: Vec<PersonEntity> = get_people(&db).await?;

    // build a list of contacts
    let mut contacts: Vec<Contact> = Vec::new();

    // build contacts
    for p in people {
        let address = get_address(&db, p.address_id).await?;
        let phones = get_phones(&db, p.person_id).await?;
        let emails = get_emails(&db, p.person_id).await?;

        let contact = contact_builder(p, address, phones, emails);
        contacts.push(contact);
    }

    Ok(Json(contacts))
}

#[get("/read/id/<id>")]
pub async fn query_by_id(db: Db, id: i32) -> Result<Json<Vec<Contact>>> {
    // return the first person with the given id
    let p: PersonEntity = get_person_by_id(&db, id).await?;
    let address = get_address(&db, p.address_id).await?;
    let phones = get_phones(&db, p.person_id).await?;
    let emails = get_emails(&db, p.person_id).await?;
    let contact = vec![contact_builder(p.clone(), address, phones, emails)];

    Ok(Json(contact))
}

#[get("/read/name/<name>")]
pub async fn query_by_name(db: Db, name: String) -> Result<Json<Vec<Contact>>> {
    // return first person found with the given name
    // uses LIKE for fuzzy matching
    let people: Vec<PersonEntity> = get_people_by_name(&db, name).await?;

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

#[get("/read/trash")]
async fn query_trash(db: Db) -> Result<Json<Vec<Contact>>> {
    // select all from people table
    let people: Vec<PersonEntity> = get_people_in_trash(&db).await?;

    // build a list of contacts
    let mut contacts: Vec<Contact> = Vec::new();

    // build contacts
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
        rocket.mount(
            "/api",
            routes![list, query_by_id, query_by_name, query_trash],
        )
    })
}
