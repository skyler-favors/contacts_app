use serde::{de::DeserializeOwned, Deserialize, Serialize};
use reqwest::Url;

// Error used for fetch requestes
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    RequestError,
    DeserializeError,
}

// used for get requests
pub async fn fetch<T>(url: String) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let response = reqwest::get(url).await;

    if let Ok(data) = response {
        if let Ok(repo) = data.json::<T>().await {
            Ok(repo)
        } else {
            Err(Error::DeserializeError)
        }
    } else {
        Err(Error::RequestError)
    }
}

// used for post requests
pub async fn post(url: String, contact: Contact) -> Result<(), reqwest::Error> {
    let host = Url::parse(&url).unwrap();
    let client = reqwest::Client::new();
    client
        .post(host)
        .json(&contact)
        .send()
        .await?;

    Ok(())
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

