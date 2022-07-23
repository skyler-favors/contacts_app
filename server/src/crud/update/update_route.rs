use diesel::dsl::not;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};
use rocket::fairing::AdHoc;
use rocket::form::Form;
use rocket::response::Redirect;

use crate::crud::shared::{ContactForm, Db, Result};
use crate::models::{AddressEntity, PersonEntity};

use super::update_helper::{update_address, update_emails, update_person, update_phones};
use crate::crud::read::read_helper::{
    get_address, get_email_ents, get_person_by_id, get_phone_ents,
};

#[post("/update/form/id/<id>", data = "<new_contact>")]
async fn update_form_by_id(db: Db, id: i32, new_contact: Form<ContactForm>) -> Result<Redirect> {
    let person_ent = get_person_by_id(&db, id).await?;
    let _address_ent = get_address(&db, person_ent.address_id).await?;
    let phone_number_ents = get_phone_ents(&db, id).await?;
    let email_ents = get_email_ents(&db, id).await?;

    let address_id_copy = person_ent.address_id;

    let person_ent = update_person(&new_contact, address_id_copy);
    let address_ent = update_address(&new_contact);

    let new_phones = new_contact.phone_numbers[0]
        .split(' ')
        .map(|s| s.to_string().to_lowercase())
        .collect::<Vec<String>>();
    let old_phones = phone_number_ents
        .iter()
        .map(|x| x.num.clone().to_lowercase())
        .collect::<Vec<String>>();
    let _phone_number_ents = update_phones(&db, new_phones, &old_phones, id);

    let new_emails = new_contact.emails[0]
        .split(' ')
        .map(|s| s.to_string().to_lowercase())
        .collect::<Vec<String>>();
    let old_emails = email_ents
        .iter()
        .map(|x| x.email.clone().to_lowercase())
        .collect::<Vec<String>>();
    let _email_ents = update_emails(&db, new_emails, &old_emails, id);

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

    Ok(Redirect::to(uri!("/")))
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
        rocket.mount("/api", routes![update_form_by_id, toggle_fav_by_id])
    })
}
