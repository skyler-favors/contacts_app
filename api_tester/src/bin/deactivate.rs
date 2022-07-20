use api_tester::deactivate_request;
use std::env;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args: Vec<String> = env::args().collect();
    let id = args[1].parse::<i32>().unwrap();

    deactivate_request(id).await?;
    println!("Deactivated {}", id);

    Ok(())
}

