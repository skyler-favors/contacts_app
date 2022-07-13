use rocket::serde::json::Json;
use rocket::fairing::AdHoc;
use rocket::response::Debug;
use rocket_sync_db_pools::diesel;
use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

use crate::models::{PersonEntity, PhoneLinkEntity, EmailLinkEntity, AddressEntity};
use crate::schema::*;

#[database("diesel")]
struct Db(diesel::PgConnection);

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[derive(Deserialize, Serialize)]
struct Contact {
    // person table
    firstname: String,
    lastname: String,
    nickname: String,
    company: String,
    url: String,
    notes: String,
    favorite: bool,
    active: bool,

    // address table
    street: String,
    city: String,
    state: String,
    zip: String,
    country: String,

    // email table
    emails: Vec<String>,

    // phone number table
    phone_numbers: Vec<String>,
}

#[get("/")]
async fn list(db: Db) -> Result<Json<Vec<Contact>>> {
    // SELECT * FROM people
    let people: Vec<PersonEntity> = db.run(move |conn| {
        people::table.load(conn)
    }).await?;

    // Build vector of Contacts
    let mut contacts: Vec<Contact> = Vec::new();
    for person in people  {
        // SELECT * FROM addresses WHERE address_id = person.address_id
        let person_1 = person.clone();
        let address: AddressEntity = db.run(move |conn| {
            addresses::table
                .filter(addresses::address_id.eq(person_1.address_id))
                .first(conn)
        }).await?;

        // SELECT * FROM phone_link WHERE person_id = person.person_id
        let person_2 = person.clone();
        let phone_number_ids: Vec<PhoneLinkEntity> = db.run(move |conn| {
            phone_link::table
                .filter(phone_link::person_id.eq(person_2.person_id))
                .load(conn)
        }).await?;

        // SELECT * FROM email_link WHERE person_id = person.person_id
        let person_3 = person.clone();
        let email_ids: Vec<EmailLinkEntity> = db.run(move |conn| {
            emails_link::table
                .filter(emails_link::person_id.eq(person_3.person_id))
                .load(conn)
        }).await?;

        // Build vector of all phone numbers from phone_number_ids
        let mut phone_numbers: Vec<String> = Vec::new();
        for id in phone_number_ids {
            // SELECT num FROM phone_numbers WHERE phone_id = id.phone_id
            let phone_number: String = db.run(move |conn| {
                phone_numbers::table
                    .filter(phone_numbers::phone_id.eq(id.phone_id))
                    .select(phone_numbers::num)
                    .first(conn)
            }).await?;
            phone_numbers.push(phone_number);
        }

        // Do the same thing above but for email ids
        let mut emails: Vec<String> = Vec::new();
        for id in email_ids {
            // SELECT email FROM emails WHERE email_id = id.email_id
            let email: String = db.run(move |conn| {
                emails::table
                    .filter(emails::email_id.eq(id.email_id))
                    .select(emails::email)
                    .first(conn)
            }).await?;
            emails.push(email);
        }

        let contact = Contact {
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
            country:address.country, 

            emails: emails.clone(),

            phone_numbers: phone_numbers.clone(),
        };
        contacts.push(contact);
    }
    Ok(Json(contacts))
}

// the form for creating the user
//#[get("/add")]
//async fn create_user() -> Option<NamedFile> {
//    NamedFile::open("create_acc.html").await.ok()
//}

// displays the created user
//#[post("/add", data="<user>")]
//async fn created_user(db: Db, user: Form<User>) -> Result<Created<String>> {
//    let new_user = User {
//        name: user.name.clone(),
//        email: user.email.clone(),
//        age: user.age,
//    };
//
//    db.run(move |conn| {
//        diesel::insert_into(users::table).values(new_user).execute(conn)
//    }).await?;
//
//    let result = format!("User: {}, {} {}", user.name, user.email, user.age);
//
//    Ok(Created::new("/create").body(result))
//
//}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel Stage", |rocket| async {
        rocket
            .attach(Db::fairing())
            .mount("/", routes![list])
    })
}
