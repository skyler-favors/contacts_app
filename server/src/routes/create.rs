use rocket::serde::json::Json;
use rocket::fairing::AdHoc;
use rocket_sync_db_pools::diesel;
use diesel::prelude::*;
use rocket::response::status::Created;
use rocket::response::Debug;

use crate::models::{Person, Address, Email, PhoneNumber};
use crate::schema::*;
use crate::helper::shared::Contact;
use crate::*;

#[database("diesel")]
struct Db(diesel::PgConnection);
type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[post("/", data = "<contact>")]
async fn create(db: Db, contact: Json<Contact>) -> Result<Created<Json<Contact>>> {
    // create address insert
    let address = Address {
        street: contact.street.clone().to_lowercase(),
        city: contact.city.clone().to_lowercase(),
        state: contact.state.clone().to_lowercase(),
        zip: contact.zip.clone().to_lowercase(),
        country: contact.country.clone().to_lowercase(),
    };

    // insert address and return new address_id
    let address_id: Vec<i32> = db.run(move |conn| {
        diesel::insert_into(addresses::table)
            .values(address)
            .returning(addresses::address_id)
            .get_results(conn)
    }).await?;

    // same thing, create person for insert
    let person = Person {
        firstname: contact.firstname.clone().to_lowercase(),
        lastname: contact.lastname.clone().to_lowercase(),
        nickname: contact.nickname.clone().to_lowercase(),
        company: contact.company.clone().to_lowercase(),
        url: contact.url.clone().to_lowercase(),
        notes: contact.notes.clone().to_lowercase(),
        favorite: contact.favorite,
        active: contact.active,
        address_id: address_id[0],
    };

    // insert and return person id
    let person_id: Vec<i32> = db.run(move |conn| {
        diesel::insert_into(people::table)
            .values(person)
            .returning(people::person_id)
            .get_results(conn)
    }).await?;

    // insert each email in the vec of emails
    for e in &contact.emails {
        let email = Email {
            person_id: person_id[0],
            email: e.clone().to_lowercase(),
        };

        // insert email
        db.run(move |conn| {
            diesel::insert_into(emails::table)
                .values(email)
                .execute(conn)
        }).await?;
    }

    // same thing as above but for phone numbers
    for p in &contact.phone_numbers {
        let phone_number = PhoneNumber {
            person_id: person_id[0],
            num: p.clone().to_lowercase(),
        };

        db.run(move |conn| {
            diesel::insert_into(phone_numbers::table)
                .values(phone_number)
                .execute(conn)
        }).await?;
    }

    Ok(Created::new("/").body(contact))
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel Stage", |rocket| async {
        rocket
            .attach(Db::fairing())
            .mount("/", routes![create])
    })
}
