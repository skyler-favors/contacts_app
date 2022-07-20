use api_tester::get_request;
use std::env;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args: Vec<String> = env::args().collect();
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
    let body = get_request(id).await?;
    println!("{:?}", body);

    Ok(())
}
