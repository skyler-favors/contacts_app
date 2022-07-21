use serde::{de::DeserializeOwned, Deserialize, Serialize};
//use reqwest::Url;

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

pub async fn toggle_delete_true(id: i32) -> Result<(), Error> {
    let body = reqwest::get(format!("http://localhost:8000/api/toggle/true/id/{}", id))
        .await;
    match body {
        Ok(_x) => {
            Ok(())
        },
        Err(_e) => {
            Err(Error::RequestError)
        }
    }
}

pub async fn toggle_delete_false(id: i32) -> Result<(), Error> {
    let body = reqwest::get(format!("http://localhost:8000/api/toggle/false/id/{}", id))
        .await;
    match body {
        Ok(_x) => {
            Ok(())
        },
        Err(_e) => {
            Err(Error::RequestError)
        }
    }
}

// used for post requests
/* pub async fn post(url: String, contact: Contact) -> Result<(), reqwest::Error> {
    let host = Url::parse(&url).unwrap();
    let client = reqwest::Client::new();
    client
        .post(host)
        .json(&contact)
        .send()
        .await?;

    Ok(())
} */

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Contact {
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

