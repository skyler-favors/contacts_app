use api_tester::{Contact, get_request, update_request};
use std::env;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let id = 84;
    let new_name = "test";
    let target: Vec<Contact> = get_request(id).await?;
    let mut target = target[0].clone();

    target.firstname = new_name.to_string();
    println!("changed {} to {}", target.firstname, new_name);

    update_request(target).await?;

    Ok(())
}
