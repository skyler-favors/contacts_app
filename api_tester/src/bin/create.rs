use api_tester::{load_local_json, post_request};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let local = load_local_json("generated.json").expect("Error loading local json data");
    post_request(local).await?;
    Ok(())
}

