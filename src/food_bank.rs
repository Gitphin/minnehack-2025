use dotenv::dotenv;
use reqwest::Client;
use serde_json::Value;
use crate::{gcloud, helper_funcs};

pub(crate) async fn find_food_shelters(client: Client) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let google_maps_api_key = gcloud::get_gmaps_api_key()?;

    // User's current location TODO: Change so passed as argument when getting location.
    let user_lat = 40.7128;
    let user_lon = -74.0060;

    let url = format!(
        "https://maps.googleapis.com/maps/api/place/nearbysearch/json?location={},{}&radius=500&keyword=food%20bank|soup%20kitchen|food%20pantry&key={}",
        user_lat, user_lon, google_maps_api_key
    );

    let response = client.get(&url).send().await?.json::<Value>().await?;

    // Extract food shelter details
    if let Some(results) = response["results"].as_array() {
        for shelter in results {
            let name = shelter["name"].as_str().unwrap_or("Unknown");
            let address = shelter["vicinity"].as_str().unwrap_or("No address available");

            // Extract shelter's lat/lng
            let shelter_lat = shelter["geometry"]["location"]["lat"].as_f64().unwrap_or(0.0);
            let shelter_lon = shelter["geometry"]["location"]["lng"].as_f64().unwrap_or(0.0);

            // Calculate distance using Haversine formula
            let distance = helper_funcs::haversine_distance(user_lat, user_lon, shelter_lat, shelter_lon);

            println!(
                "Name: {}\nLocation: {}, {}\nAddress: {}\nDistance: {:.2} km\n",
                name, shelter_lat, shelter_lon, address, distance
            );
        }
    } else {
        println!("No food shelters found.");
    }

    Ok(())
}
