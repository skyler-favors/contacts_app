use std::{path::Path, fs::File};
use std::io::BufReader;

use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
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

fn get_data() -> Result<Vec<Contact>, serde_json::Error> {
    let path = Path::new("/Users/sfav/Downloads/generated.json");
    let file = File::open(path).unwrap();
    let buf = BufReader::new(file);
    let contacts: Vec<Contact> = serde_json::from_reader(buf)?;
    Ok(contacts)
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let contacts = get_data().ok().unwrap();
    let host = Url::parse("http://127.0.0.1:8000/").unwrap();
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
