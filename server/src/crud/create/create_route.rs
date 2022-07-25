use rocket::fairing::AdHoc;
use rocket::form::Form;
use rocket::{response::Redirect, serde::json::Json};
use serde::{Serialize, Deserialize};

use super::create_helper::{insert_addresses, insert_emails, insert_person, insert_phone_numbers};
use crate::crud::shared::{ContactForm, Db, Result};

#[derive(Serialize, Deserialize)]
struct Id {
    id: i32,
}

#[post("/create/json", data = "<contact>")]
async fn create_from_json(db: Db, contact: Json<ContactForm>) -> Result<Json<Id>> {
    let address_id = insert_addresses(&db, &contact).await?;
    let person_id = insert_person(&db, &contact, address_id).await?;

    insert_emails(&db, &contact.emails, person_id).await?;
    insert_phone_numbers(&db, &contact.phone_numbers, person_id).await?;
    println!("{:?}", &contact.phone_numbers);

    Ok(Json(Id {id: person_id}))
}

#[post("/create/form", data = "<contact>")]
async fn create_from_form(db: Db, contact: Form<ContactForm>) -> Result<Redirect> {
    let address_id = insert_addresses(&db, &contact).await?;
    let person_id = insert_person(&db, &contact, address_id).await?;

    let emails = contact.emails[0]
        .split(' ')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let phone_numbers = contact.phone_numbers[0]
        .split(' ')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    insert_emails(&db, &emails, person_id).await?;
    insert_phone_numbers(&db, &phone_numbers, person_id).await?;

    Ok(Redirect::to(uri!("/")))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel Stage", |rocket| async {
        rocket.mount("/api", routes![create_from_form, create_from_json])
    })
}
