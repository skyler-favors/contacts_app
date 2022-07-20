use api_tester::delete_request;
use std::env;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args: Vec<String> = env::args().collect();
    let id = args[1].parse::<i32>().unwrap();

    delete_request(id).await?;
    println!("deleted {}", id);

    Ok(())
}

