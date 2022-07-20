use api_tester::{Contact, get_request, update_request};
use std::env;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args: Vec<String> = env::args().collect();
    let new_name = &args[2];

    let args = args[1].parse::<i32>();
    let mut id = 1;
    match args {
        Ok(x) => {
            id = x;
        },
        Err(e) => {
            println!("Invalid argument; continuing with ID of 1: {}", e);
        }
    }

    let target: Vec<Contact> = get_request(id).await?;
    let mut target = target[0].clone();

    target.firstname = new_name.to_string();
    println!("changed {} to {}", target.firstname, new_name);

    let _result = update_request(target).await?;

    Ok(())
}
