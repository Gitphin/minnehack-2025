use reqwest::Client;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    
    let res = client.get("http://localhost:3001/find_restaurants")
        .send()
        .await?
        .text()
        .await?;
    
    println!("JSON Response:\n{}", res);
    
    Ok(())
}
