use dotenv::dotenv;
use std::env;
use reqwest::Client;

pub(crate) fn init() -> Result<Client, Box<dyn std::error::Error + Send + Sync>> {
    let my_client = Client::new();
    Ok(my_client)
}

pub(crate) fn get_gmaps_api_key() -> Result<String, String> {
    dotenv().ok();
    match env::var("GCLOUD_MAPS_API_KEY") {
        Ok(gmaps_key) => Ok(gmaps_key),
        Err(_) => Err("Missing env variable: GCLOUD_MAPS_API_KEY".to_string()),
    }
}

