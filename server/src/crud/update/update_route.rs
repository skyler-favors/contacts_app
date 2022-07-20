use diesel::{QueryDsl, RunQueryDsl};
use rocket::fairing::AdHoc;
use rocket::serde::json::Json;

use crate::crud::shared::{Contact, Db, Result};
use crate::models::{PersonEntity, AddressEntity, EmailEntity, PhoneNumberEntity};

use super::update::{update_person, update_address, update_phones, update_emails};
use crate::crud::read::read::{get_person_by_id, get_address, get_phone_ents, get_email_ents};

#[post("/update/id/<id>", data = "<new_contact>")]
async fn update_by_id(db: Db, id: i32, new_contact: Json<Contact>) -> Result<()> {
    let person_ent = get_person_by_id(&db, id).await?;
    let _address_ent = get_address(&db, person_ent.address_id).await?;
    let _phone_number_ents = get_phone_ents(&db, id).await?;
    let _email_ents = get_email_ents(&db, id).await?;

    let address_id_copy = person_ent.address_id;

    let person_ent = update_person(&new_contact, address_id_copy);
    let address_ent = update_address(&new_contact);
    let phone_number_ents = update_phones(&new_contact.phone_numbers, id);
    let email_ents = update_emails(&new_contact.emails, id);

    
    use crate::schema::people::dsl::*;
    let _person_result: PersonEntity = db.run(move |conn| {
        diesel::update(people.find(id))
            .set(person_ent)
            .get_result(conn)
    }).await?;

    use crate::schema::addresses::dsl::*;
    let _address_result: AddressEntity = db.run(move |conn| {
        diesel::update(addresses.find(address_id_copy))
            .set(address_ent)
            .get_result(conn)
    }).await?;

    use crate::schema::emails::dsl::*;
    for email_ent in email_ents {
        let _email_result: EmailEntity = db.run(move |conn| {
            diesel::update(emails.find(id))
                .set(email_ent)
                .get_result(conn)
        }).await?;
    }

    use crate::schema::phone_numbers::dsl::*;
    for phone_ent in phone_number_ents {
        let _phone_result: PhoneNumberEntity = db.run(move |conn| {
            diesel::update(phone_numbers.find(id))
                .set(phone_ent)
                .get_result(conn)
        }).await?;
    }

    Ok(())
}

//#[post("/update/name/<name>", data = "<contact>")]
//async fn update_by_name(db: Db, name: String, contact: Json<Contact>) -> Result<Json<Vec<Contact>>> {
//    // return first person found with the given name
//    // uses LIKE for fuzzy matching
//    let people: Vec<PersonEntity> = get_people_by_name(&db, name).await?;
//
//    let mut contacts: Vec<Contact> = Vec::new();
//    for p in people {
//        let address = get_address(&db, p.address_id).await?;
//        let phones = get_phones(&db, p.person_id).await?;
//        let emails = get_emails(&db, p.person_id).await?;
//        let contact = contact_builder(p, address, phones, emails);
//        contacts.push(contact);
//    }
//
//    Ok(Json(contacts))
//}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel Stage", |rocket| async {
        rocket
            .mount("/api", routes![update_by_id])
    })
}
