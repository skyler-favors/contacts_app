use diesel::prelude::*;
use crate::crud::shared::{Contact, Db, Result};
use crate::models::{AddressEntity, EmailEntity, PersonEntity, PhoneNumberEntity};
use crate::schema::*;

// get address info for person
pub async fn get_address(db: &Db, address_id: i32) -> Result<AddressEntity> {
    // SELECT * FROM addresses WHERE address_id = person.address_id
    let address: AddressEntity = db
        .run(move |conn| {
            addresses::table
                .filter(addresses::address_id.eq(address_id))
                .first(conn)
        })
        .await?;

    Ok(address)
}

// get all phones for 1 person
pub async fn get_phones(db: &Db, person_id: i32) -> Result<Vec<String>> {
    // SELECT num FROM phone_numbers WHERE phone_id = id.phone_id
    let phone_numbers: Vec<String> = db
        .run(move |conn| {
            phone_numbers::table
                .filter(phone_numbers::person_id.eq(person_id))
                .select(phone_numbers::num)
                .load(conn)
        })
        .await?;

    Ok(phone_numbers)
}

pub async fn get_phone_ents(db: &Db, person_id: i32) -> Result<Vec<PhoneNumberEntity>> {
    // SELECT num FROM phone_numbers WHERE phone_id = id.phone_id
    let phone_numbers: Vec<PhoneNumberEntity> = db
        .run(move |conn| {
            phone_numbers::table
                .filter(phone_numbers::person_id.eq(person_id))
                .load(conn)
        })
        .await?;

    Ok(phone_numbers)
}

// get all emails for 1 person
pub async fn get_emails(db: &Db, person_id: i32) -> Result<Vec<String>> {
    // SELECT email FROM emails WHERE email_id = id.email_id
    let emails: Vec<String> = db
        .run(move |conn| {
            emails::table
                .filter(emails::person_id.eq(person_id))
                .select(emails::email)
                .load(conn)
        })
        .await?;

    Ok(emails)
}

pub async fn get_email_ents(db: &Db, person_id: i32) -> Result<Vec<EmailEntity>> {
    // SELECT email FROM emails WHERE email_id = id.email_id
    let emails: Vec<EmailEntity> = db
        .run(move |conn| {
            emails::table
                .filter(emails::person_id.eq(person_id))
                .load(conn)
        })
        .await?;

    Ok(emails)
}

// select a specific person
pub async fn get_person_by_id(db: &Db, person_id: i32) -> Result<PersonEntity> {
    let person: PersonEntity = db
        .run(move |conn| {
            people::table
                .filter(people::person_id.eq(person_id))
                .first(conn)
        })
        .await?;
    Ok(person)
}

pub async fn get_people_by_name(db: &Db, name: String) -> Result<Vec<PersonEntity>> {
    let person: Vec<PersonEntity> = db
        .run(move |conn| {
            people::table
                .filter(people::firstname.like(format!("%{}%", name)))
                .load(conn)
        })
        .await?;
    Ok(person)
}

// select all from people table
pub async fn get_people(db: &Db) -> Result<Vec<PersonEntity>> {
    // SELECT * FROM people
    let people: Vec<PersonEntity> = db.run(move |conn| people::table.load(conn)).await?;

    Ok(people)
}

// select all from people table
pub async fn get_people_in_trash(db: &Db) -> Result<Vec<PersonEntity>> {
    let person: Vec<PersonEntity> = db
        .run(move |conn| {
            people::table
                .filter(people::active.eq(false))
                .load(conn)
        })
        .await?;
    Ok(person)
}

// used to easily build a contact struct
pub fn contact_builder(
    person: PersonEntity,
    address: AddressEntity,
    phone_numbers: Vec<String>,
    emails: Vec<String>,
) -> Contact {
    Contact {
        id: person.person_id,
        firstname: person.firstname,
        lastname: person.lastname,
        nickname: person.nickname,
        company: person.company,
        url: person.url,
        notes: person.notes,
        favorite: person.favorite,
        active: person.active,

        street: address.street,
        city: address.city,
        state: address.state,
        zip: address.zip,
        country: address.country,

        emails,
        phone_numbers,
    }
}
