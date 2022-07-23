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

pub async fn toggle_trash(id: i32) -> Result<(), Error> {
    let body = reqwest::get(format!("http://localhost:8000/api/toggle/trash/id/{}", id)).await;
    match body {
        Ok(_x) => Ok(()),
        Err(_e) => Err(Error::RequestError),
    }
}

// Not being used
// pub async fn delete_by_id(id: i32) -> Result<(), Error> {
//     let body = reqwest::get(format!("http://localhost:8000/api/delete/id/{}", id))
//         .await;
//     match body {
//         Ok(_x) => {
//             Ok(())
//         },
//         Err(_e) => {
//             Err(Error::RequestError)
//         }
//     }
// }

pub async fn delete_all() -> Result<(), Error> {
    let body = reqwest::get("http://localhost:8000/api/delete/all".to_string()).await;
    match body {
        Ok(_x) => Ok(()),
        Err(_e) => Err(Error::RequestError),
    }
}

pub async fn toggle_fav(id: i32) -> Result<(), Error> {
    let body = reqwest::get(format!("http://localhost:8000/api/toggle/fav/id/{}", id)).await;
    match body {
        Ok(_x) => Ok(()),
        Err(_e) => Err(Error::RequestError),
    }
}

pub async fn fetch_all() -> Result<Vec<Contact>, Error> {
    fetch::<Vec<Contact>>("http://localhost:8000/api/read/all".into()).await
}

pub async fn fetch_contact(name: String) -> Result<Vec<Contact>, Error> {
    fetch::<Vec<Contact>>(format!("http://localhost:8000/api/read/name/{}", name)).await
}

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
