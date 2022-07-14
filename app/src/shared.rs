use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// You can use reqwest or other crates to fetch your api.
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

// You can use thiserror to define your errors.
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    RequestError,
    DeserializeError,
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

