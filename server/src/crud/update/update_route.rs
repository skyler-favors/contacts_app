use diesel::dsl::not;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};
use rocket::fairing::AdHoc;
use rocket::serde::json::Json;

use crate::crud::shared::{ContactForm, Db, Result};
use crate::models::{AddressEntity, PersonEntity};

use super::update_helper::{update_address, update_emails, update_person, update_phones};
use crate::crud::read::read_helper::{
    get_address, get_email_ents, get_person_by_id, get_phone_ents,
};

#[post("/update/json/id/<id>", data = "<new_contact>")]
async fn update_json_id(db: Db, id: i32, new_contact: Json<ContactForm>) -> Result<()> {
    let person_ent = get_person_by_id(&db, id).await?;
    let _address_ent = get_address(&db, person_ent.address_id).await?;
    let phone_number_ents = get_phone_ents(&db, id).await?;
    let email_ents = get_email_ents(&db, id).await?;

    let address_id_copy = person_ent.address_id;

    let person_ent = update_person(&new_contact, address_id_copy);
    let address_ent = update_address(&new_contact);

    let new_phones = new_contact
        .phone_numbers
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string().to_lowercase())
        .collect::<Vec<String>>();

    update_phones(&db, new_phones, id).await?;

    let new_emails = new_contact.emails
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string().to_lowercase())
        .collect::<Vec<String>>();

    println!("{:?}", new_emails);
    update_emails(&db, new_emails, id).await?;

    use crate::schema::people::dsl::*;
    let _person_result: PersonEntity = db
        .run(move |conn| {
            diesel::update(people.find(id))
                .set(person_ent)
                .get_result(conn)
        })
        .await?;

    use crate::schema::addresses::dsl::*;
    let _address_result: AddressEntity = db
        .run(move |conn| {
            diesel::update(addresses.find(address_id_copy))
                .set(address_ent)
                .get_result(conn)
        })
        .await?;

    Ok(())
}

#[get("/toggle/fav/id/<id>")]
async fn toggle_fav_by_id(db: Db, id: i32) -> Result<()> {
    use crate::schema::people::dsl::*;
    let _result: PersonEntity = db
        .run(move |conn| {
            diesel::update(people.find(id))
                .set(favorite.eq(not(favorite)))
                .get_result(conn)
        })
        .await?;

    Ok(())
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel Stage", |rocket| async {
        rocket.mount("/api", routes![update_json_id, toggle_fav_by_id])
    })
}
