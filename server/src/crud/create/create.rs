use crate::crud::shared::{Contact, Result, Db};
use rocket::serde::json::Json;
use crate::models::{Person, Address, Email, PhoneNumber};
use crate::schema::*;
use diesel::prelude::*;

pub async fn insert_addresses(db: &Db, contact: &Json<Contact>) -> Result<i32> {
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

    Ok(address_id[0])
}

pub async fn insert_person(db: &Db, contact: &Json<Contact>, address_id: i32) -> Result<i32> {
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
        address_id,
    };

    // insert and return person id
    let person_id: Vec<i32> = db.run(move |conn| {
        diesel::insert_into(people::table)
            .values(person)
            .returning(people::person_id)
            .get_results(conn)
    }).await?;

    Ok(person_id[0])
}

pub async fn insert_phone_numbers(db: &Db, contact: &Json<Contact>, person_id: i32) -> Result<()> {
   // insert each email in the vec of emails
    for e in &contact.emails {
        let email = Email {
            person_id,
            email: e.clone().to_lowercase(),
        };

        // insert email
        db.run(move |conn| {
            diesel::insert_into(emails::table)
                .values(email)
                .execute(conn)
        }).await?;
    }

    Ok(())
}

pub async fn insert_emails(db: &Db, contact: &Json<Contact>, person_id: i32) -> Result<()> {
    // same thing as above but for phone numbers
    for p in &contact.phone_numbers {
        let phone_number = PhoneNumber {
            person_id,
            num: p.clone().to_lowercase(),
        };

        db.run(move |conn| {
            diesel::insert_into(phone_numbers::table)
                .values(phone_number)
                .execute(conn)
        }).await?;
    }

    Ok(())
}
