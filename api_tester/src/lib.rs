use std::{path::Path, fs::File};
use std::io::BufReader;
use std::env;

use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Contact {
    // person table
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

pub async fn get_request(id: i32) -> Result<Vec<Contact>, reqwest::Error> {
    let body: Vec<Contact> = reqwest::get(format!("http://localhost:8000/api/read/id/{}", id))
        .await?
        .json()
        .await?;
    Ok(body)
}

pub async fn delete_request(id: i32) -> Result<(), reqwest::Error> {
    let _body = reqwest::get(format!("http://localhost:8000/api/delete/id/{}", id))
        .await?;
    Ok(())
}

pub async fn deactivate_request(id: i32) -> Result<(), reqwest::Error> {
    let _body = reqwest::get(format!("http://localhost:8000/api/deactivate/id/{}", id))
        .await?;
    Ok(())
}

pub fn load_local_json(filename: &str) -> Result<Vec<Contact>, serde_json::Error> {
    let path = Path::new(filename);
    let curr_path = env::current_dir().unwrap().join(path);
    //println!("{:?}", curr_path);

    let file = File::open(curr_path).unwrap();
    let buf = BufReader::new(file);
    let contacts: Vec<Contact> = serde_json::from_reader(buf)?;
    Ok(contacts)
}

pub async fn post_request(contacts: Vec<Contact>) -> Result<(), reqwest::Error> {
    let host = Url::parse("http://127.0.0.1:8000/api/create").unwrap();
    let client = reqwest::Client::new();

    for (i, c) in contacts.iter().enumerate() {
        client
            .post(host.clone())
            .json(c)
            .send()
            .await?;
        println!("sent: {}", i);
    }

    Ok(())
}

pub async fn update_request(new_data: Contact) -> Result<(), reqwest::Error> {
    let host = Url::parse("http://127.0.0.1:8000/api/update/id/1").unwrap();
    let client = reqwest::Client::new();

    client
        .post(host.clone())
        .json(&new_data)
        .send()
        .await?;

    Ok(())
}
