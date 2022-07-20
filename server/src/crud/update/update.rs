use crate::crud::shared::Contact;
use crate::models::{Person, Address, Email, PhoneNumber};

pub fn update_person(new_contact: &Contact, address_id: i32) -> Person {
    Person {
        firstname: new_contact.firstname.clone().to_lowercase(),
        lastname: new_contact.lastname.clone().to_lowercase(),
        nickname: new_contact.nickname.clone().to_lowercase(),
        company: new_contact.company.clone().to_lowercase(),
        url: new_contact.url.clone().to_lowercase(),
        notes: new_contact.notes.clone().to_lowercase(),
        favorite: new_contact.favorite,
        active: new_contact.active,
        address_id,
    }
}

pub fn update_address(new_contact: &Contact) -> Address {
    Address {
        street: new_contact.street.clone().to_lowercase(),
        city: new_contact.city.clone().to_lowercase(),
        state: new_contact.state.clone().to_lowercase(),
        zip: new_contact.zip.clone().to_lowercase(),
        country: new_contact.country.clone().to_lowercase(),
    }
}

pub fn update_emails(new_emails: &Vec<String>, person_id: i32) -> Vec<Email> {
    let mut result: Vec<Email> = Vec::new();
    for e in new_emails {
        let email = Email {
            person_id,
            email: e.clone().to_lowercase(),
        };
        result.push(email);
    }
    result
}

pub fn update_phones(new_phones: &Vec<String>, person_id: i32) -> Vec<PhoneNumber> {
    let mut result: Vec<PhoneNumber> = Vec::new();
    for p in new_phones {
        let phone = PhoneNumber {
            person_id,
            num: p.clone().to_lowercase(),
        };
        result.push(phone);
    }
    result
}
