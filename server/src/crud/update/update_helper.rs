use crate::crud::shared::{ContactForm, Db};
use crate::models::{Address, Email, Person, PhoneNumber};
use crate::schema::*;
use diesel::prelude::*;
use diesel::result::Error;

pub fn update_person(new_contact: &ContactForm, address_id: i32) -> Person {
    Person {
        firstname: new_contact.firstname.clone().to_lowercase(),
        lastname: {
            new_contact
                .lastname
                .as_ref()
                .map(|x| x.clone().to_lowercase())
        },
        nickname: {
            new_contact
                .nickname
                .as_ref()
                .map(|x| x.clone().to_lowercase())
        },
        company: {
            new_contact
                .company
                .as_ref()
                .map(|x| x.clone().to_lowercase())
        },
        url: { new_contact.url.as_ref().map(|x| x.clone().to_lowercase()) },
        notes: { new_contact.notes.as_ref().map(|x| x.clone().to_lowercase()) },
        favorite: new_contact.favorite,
        active: new_contact.active,
        address_id,
    }
}

pub fn update_address(new_contact: &ContactForm) -> Address {
    Address {
        street: {
            new_contact
                .street
                .as_ref()
                .map(|x| x.clone().to_lowercase())
        },
        city: { new_contact.city.as_ref().map(|x| x.clone().to_lowercase()) },
        state: { new_contact.state.as_ref().map(|x| x.clone().to_lowercase()) },
        zip: { new_contact.zip.as_ref().map(|x| x.clone().to_lowercase()) },
        country: {
            new_contact
                .country
                .as_ref()
                .map(|x| x.clone().to_lowercase())
        },
    }
}

pub async fn update_emails(
    db: &Db,
    new_emails: Vec<String>,
    id: i32,
) -> Result<(), Error> {
    let mut result: Vec<Email> = Vec::new();

    for e in new_emails {
        let email_ent = Email {
            person_id: id,
            email: e.clone().to_lowercase(),
        };
        result.push(email_ent);
    }

    // delete all old emails and insert new ones
    db.run(move |conn| {
        diesel::delete(emails::table)
            .filter(emails::person_id.eq(id))
            .execute(conn)
    })
    .await?;

    for email_ent in result {
        let _email_result = db
            .run(move |conn| {
                diesel::insert_into(emails::table)
                    .values(email_ent)
                    .execute(conn)
            })
            .await?;
    }

    Ok(())
}

pub async fn update_phones(
    db: &Db,
    new_phones: Vec<String>,
    id: i32,
) -> Result<(), Error> {
    println!("{:?}", new_phones);
    let mut result: Vec<PhoneNumber> = Vec::new();
    for p in new_phones {
        let phone = PhoneNumber {
            person_id: id,
            num: p.clone().to_lowercase(),
        };
        result.push(phone);
    }

    db.run(move |conn| {
        diesel::delete(phone_numbers::table)
            .filter(phone_numbers::person_id.eq(id))
            .execute(conn)
    })
    .await?;

    for phone_ent in result {
        println!("{:?}", phone_ent.num);
        let _phone_result = db
            .run(move |conn| {
                diesel::insert_into(phone_numbers::table)
                    .values(phone_ent)
                    .execute(conn)
            })
            .await?;
    }

    Ok(())
}
