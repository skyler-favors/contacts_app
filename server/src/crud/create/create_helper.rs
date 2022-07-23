use crate::crud::shared::{ContactForm, Db, Result};
use crate::models::{Address, Email, Person, PhoneNumber};
use crate::schema::*;
use diesel::prelude::*;

pub async fn insert_addresses(db: &Db, contact: &ContactForm) -> Result<i32> {
    // need to create a way to check if all fields are null
    // create address insert
    let address = Address {
        street: {
            match &contact.street {
                Some(x) => {
                    if x.is_empty() {
                        None
                    } else {
                        Some(x.clone().to_lowercase())
                    }
                }
                None => None,
            }
        },
        city: {
            match &contact.city {
                Some(x) => {
                    if x.is_empty() {
                        None
                    } else {
                        Some(x.clone().to_lowercase())
                    }
                }
                None => None,
            }
        },
        state: {
            match &contact.state {
                Some(x) => {
                    if x.is_empty() {
                        None
                    } else {
                        Some(x.clone().to_lowercase())
                    }
                }
                None => None,
            }
        },
        zip: {
            match &contact.zip {
                Some(x) => {
                    if x.is_empty() {
                        None
                    } else {
                        Some(x.clone().to_lowercase())
                    }
                }
                None => None,
            }
        },
        country: {
            match &contact.country {
                Some(x) => {
                    if x.is_empty() {
                        None
                    } else {
                        Some(x.clone().to_lowercase())
                    }
                }
                None => None,
            }
        },
    };

    // insert address and return new address_id
    let address_id: Vec<i32> = db
        .run(move |conn| {
            diesel::insert_into(addresses::table)
                .values(address)
                .returning(addresses::address_id)
                .get_results(conn)
        })
        .await?;

    Ok(address_id[0])
}

pub async fn insert_person(db: &Db, contact: &ContactForm, address_id: i32) -> Result<i32> {
    // same thing, create person for insert
    let person = Person {
        firstname: contact.firstname.clone().to_lowercase(),
        lastname: {
            match &contact.lastname {
                Some(x) => {
                    if x.is_empty() {
                        None
                    } else {
                        Some(x.clone().to_lowercase())
                    }
                }
                None => None,
            }
        },
        nickname: {
            match &contact.nickname {
                Some(x) => {
                    if x.is_empty() {
                        None
                    } else {
                        Some(x.clone().to_lowercase())
                    }
                }
                None => None,
            }
        },
        company: {
            match &contact.company {
                Some(x) => {
                    if x.is_empty() {
                        None
                    } else {
                        Some(x.clone().to_lowercase())
                    }
                }
                None => None,
            }
        },
        url: {
            match &contact.url {
                Some(x) => {
                    if x.is_empty() {
                        None
                    } else {
                        Some(x.clone().to_lowercase())
                    }
                }
                None => None,
            }
        },
        notes: {
            match &contact.notes {
                Some(x) => {
                    if x.is_empty() {
                        None
                    } else {
                        Some(x.clone().to_lowercase())
                    }
                }
                None => None,
            }
        },
        favorite: contact.favorite,
        active: contact.active,
        address_id,
    };

    // insert and return person id
    let person_id: Vec<i32> = db
        .run(move |conn| {
            diesel::insert_into(people::table)
                .values(person)
                .returning(people::person_id)
                .get_results(conn)
        })
        .await?;

    Ok(person_id[0])
}

pub async fn insert_phone_numbers(db: &Db, phones: &Vec<String>, person_id: i32) -> Result<()> {
    // insert each email in the vec of emails
    for e in phones {
        let phone_number = PhoneNumber {
            person_id,
            num: e.clone().to_lowercase(),
        };

        // insert email
        db.run(move |conn| {
            diesel::insert_into(phone_numbers::table)
                .values(phone_number)
                .execute(conn)
        })
        .await?;
    }
    Ok(())
}

pub async fn insert_emails(db: &Db, emails: &Vec<String>, person_id: i32) -> Result<()> {
    // same thing as above but for phone numbers
    for p in emails {
        let email = Email {
            person_id,
            email: p.clone().to_lowercase(),
        };

        db.run(move |conn| {
            diesel::insert_into(emails::table)
                .values(email)
                .execute(conn)
        })
        .await?;
    }

    Ok(())
}
