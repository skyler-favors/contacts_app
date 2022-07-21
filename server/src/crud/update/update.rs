use crate::crud::shared::Contact;
use crate::models::{Person, Address, Email, PhoneNumber};

pub fn update_person(new_contact: &Contact, address_id: i32) -> Person {
    Person {
        firstname: new_contact.firstname.clone().to_lowercase(),
        lastname: {
            match &new_contact.lastname {
                Some(x) => Some(x.clone().to_lowercase()),
                None => None,
            }
        }, 
        nickname: {
            match &new_contact.nickname {
                Some(x) => Some(x.clone().to_lowercase()),
                None => None,
            }
        },  
        company: {
            match &new_contact.company {
                Some(x) => Some(x.clone().to_lowercase()),
                None => None,
            }
        },  
        url: {
            match &new_contact.url {
                Some(x) => Some(x.clone().to_lowercase()),
                None => None,
            }
        },  
        notes: {
            match &new_contact.notes {
                Some(x) => Some(x.clone().to_lowercase()),
                None => None,
            }
        },  
        favorite: new_contact.favorite,
        active: new_contact.active,
        address_id,
    }
}

pub fn update_address(new_contact: &Contact) -> Address {
    Address {
        street: {
            match &new_contact.street {
                Some(x) => Some(x.clone().to_lowercase()),
                None => None,
            }
        }, 
        city: {
            match &new_contact.city {
                Some(x) => Some(x.clone().to_lowercase()),
                None => None,
            }
        }, 
        state: {
            match &new_contact.state {
                Some(x) => Some(x.clone().to_lowercase()),
                None => None,
            }
        }, 
        zip: {
            match &new_contact.zip {
                Some(x) => Some(x.clone().to_lowercase()),
                None => None,
            }
        }, 
        country: {
            match &new_contact.country {
                Some(x) => Some(x.clone().to_lowercase()),
                None => None,
            }
        }, 
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
