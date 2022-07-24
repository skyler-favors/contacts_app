use serde::{Deserialize, Serialize};
use std::rc::Rc;
use crate::Filter;

pub async fn fetch_all() -> Result<Rc<Vec<Rc<Contact>>>, Rc<reqwest::Error>> {
    let body: Vec<ContactNullables> = reqwest::get("http://localhost:8000/api/read/all")
        .await?
        .json()
        .await?;

    let contacts: Vec<Rc<Contact>> = body.iter().map(|x| Rc::new(Contact::new(x))).collect();

    Ok(Rc::new(contacts))
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ContactNullables {
    // person table
    pub id: i32,
    pub firstname: String,
    pub lastname: Option<String>,
    pub nickname: Option<String>,
    pub company: Option<String>,
    pub url: Option<String>,
    pub notes: Option<String>,
    pub favorite: bool,
    pub active: bool,

    // address table
    pub street: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    pub country: Option<String>,

    // email table
    pub emails: Vec<String>,

    // phone number table
    pub phone_numbers: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Contact {
    // person table
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub nickname: String,
    pub company: String,
    pub url: String,
    pub notes: String,
    pub favorite: bool,
    pub active: bool,

    // address table
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub country: String,

    // email table
    pub emails: Vec<String>,

    // phone number table
    pub phone_numbers: Vec<String>,
}

impl Contact {
    pub fn new(old_contact: &ContactNullables) -> Contact {
        let lastname = match &old_contact.lastname {
            Some(x) => x.clone(),
            None => "".to_string(),
        };
        let nickname = match &old_contact.nickname {
            Some(x) => x.clone(),
            None => "".to_string(),
        };
        let company = match &old_contact.company {
            Some(x) => x.clone(),
            None => "".to_string(),
        };
        let url = match &old_contact.url {
            Some(x) => x.clone(),
            None => "".to_string(),
        };
        let notes = match &old_contact.notes {
            Some(x) => x.clone(),
            None => "".to_string(),
        };
        let street = match &old_contact.street {
            Some(x) => x.clone(),
            None => "".to_string(),
        };
        let city = match &old_contact.city {
            Some(x) => x.clone(),
            None => "".to_string(),
        };
        let state = match &old_contact.state {
            Some(x) => x.clone(),
            None => "".to_string(),
        };
        let zip = match &old_contact.zip {
            Some(x) => x.clone(),
            None => "".to_string(),
        };
        let country = match &old_contact.country {
            Some(x) => x.clone(),
            None => "".to_string(),
        };

        Contact {
            id: old_contact.id,
            firstname: old_contact.firstname.clone(),
            lastname,
            nickname,
            company,
            url,
            notes,
            favorite: old_contact.favorite,
            active: old_contact.active,
            street,
            city,
            state,
            zip,
            country,
            emails: old_contact.emails.clone(),
            phone_numbers: old_contact.phone_numbers.clone(),
        }
    }
}

pub fn seive(mut contacts: Vec<Rc<Contact>>, filters: &Vec<Filter>) -> Vec<Rc<Contact>> {
    for filter in filters {
        contacts = match filter {
            Filter::Favorites => contacts.into_iter().filter(|x| x.favorite).collect(),
            Filter::Trash => contacts.into_iter().filter(|x| !x.active).collect(),
            Filter::Search(s) => contacts.into_iter().filter(|x| x.firstname.contains(s)).collect(),
        };
    }

    contacts
}
