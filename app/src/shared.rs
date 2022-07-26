use crate::Filter;
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use web_sys::console::log_1;

pub async fn fetch_all() -> Result<Vec<Contact>, Rc<reqwest::Error>> {
    let body: Vec<ContactNullables> = reqwest::get("http://localhost:8000/api/read/all")
        .await?
        .json()
        .await?;

    let mut contacts: Vec<Contact> = body.into_iter().map(|x| Contact::new(&x)).collect();
    contacts.sort_by_key(|c| c.firstname.clone());

    Ok(contacts)
}

pub async fn toggle_trash(id: i32) -> Result<(), Rc<reqwest::Error>> {
    reqwest::get(format!("http://localhost:8000/api/toggle/trash/id/{}", id)).await?;
    Ok(())
}

pub async fn toggle_fav(id: i32) -> Result<(), Rc<reqwest::Error>> {
    reqwest::get(format!("http://localhost:8000/api/toggle/fav/id/{}", id)).await?;
    Ok(())
}

pub async fn delete_all() -> Result<(), Rc<reqwest::Error>> {
    reqwest::get("http://localhost:8000/api/delete/all").await?;
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct Id {
    id: i32,
}

pub async fn create_contact(data: &ContactNullables) -> Result<i32, Rc<reqwest::Error>> {
    let host = "http://localhost:8000/api/create/json";
    let client = reqwest::Client::new();

    let id: Id = client
        .post(host)
        .json(&data)
        .send()
        .await?
        .json()
        .await?;

    Ok(id.id)
}

pub async fn update_contact(data: &ContactNullables) -> Result<(), Rc<reqwest::Error>> {
    let host = &format!("http://localhost:8000/api/update/json/id/{}", data.id);
    let client = reqwest::Client::new();
    let mut data = data.clone();

    data.emails = data.emails.iter().map(|s| s.chars().filter(|c| c != &',').collect::<String>()).collect();
    data.phone_numbers = data.phone_numbers.iter().map(|s| s.chars().filter(|c| c.is_numeric()).collect::<String>()).collect();

    client.post(host).json(&data).send().await?;

    Ok(())
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

impl ContactNullables {
    pub fn new(data: &Contact) -> ContactNullables {
        ContactNullables {
            id: data.id,
            firstname: data.firstname.clone(),
            lastname: Some(data.lastname.clone()),
            nickname: Some(data.nickname.clone()),
            company: Some(data.company.clone()),
            url: Some(data.url.clone()),
            notes: Some(data.notes.clone()),
            favorite: data.favorite,
            active: data.active,
            street: Some(data.street.clone()),
            city: Some(data.city.clone()),
            state: Some(data.state.clone()),
            zip: Some(data.zip.clone()),
            country: Some(data.country.clone()),
            emails: data.emails.clone(),
            phone_numbers: data.phone_numbers.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

    pub fn builder(data: Vec<String>) -> Contact {
        log_1(&format!("{:?}", data).into());
        Contact {
            firstname: data[0].clone(),
            lastname: data[1].clone(),
            nickname: data[2].clone(),
            company: data[3].clone(),
            url: data[4].clone(),
            notes: data[5].clone(),
            street: data[6].clone(),
            city: data[7].clone(),
            state: data[8].clone(),
            zip: data[9].clone(),
            country: data[10].clone(),
            emails: data[11].split(", ").map(|s| s.to_string()).collect(),
            phone_numbers: data[12].split(", ").map(|s| s.to_string()).collect(),
            id: data[13].parse::<i32>().unwrap(),
            favorite: data[14].parse::<bool>().unwrap(),
            active: data[15].parse::<bool>().unwrap(),
        }
    }

    pub fn to_vec(data: Contact) -> Vec<String> {
        let emails: String = data.emails.iter().map(|s| format!("{}, ", s)).collect();
        let phone_numbers: String = data.phone_numbers.iter().map(|s| format!("{}, ", s)).collect();
        vec![
            data.firstname,
            data.lastname,
            data.nickname,
            data.company,
            data.url,
            data.notes,
            data.street,
            data.city,
            data.state,
            data.zip,
            data.country,
            emails,
            phone_numbers,
        ]
    }
}

pub fn seive(contacts: &[Contact], filters: &Vec<Filter>) -> Vec<Contact> {
    let mut new_contacts = contacts.to_owned();
    if !filters.iter().any(|x| x == &Filter::Trash) {
        new_contacts = new_contacts
            .into_iter()
            .filter(|x| x.active)
            .collect::<Vec<Contact>>();
    }

    for filter in filters {
        new_contacts = match filter {
            Filter::Favorites => new_contacts.into_iter().filter(|x| x.favorite).collect(),
            Filter::Trash => new_contacts.into_iter().filter(|x| !x.active).collect(),
            Filter::Search(s) => new_contacts
                .into_iter()
                .filter(|x| x.firstname.contains(s))
                .collect(),
        };
    }
    new_contacts
}

