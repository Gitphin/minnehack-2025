use reqwest::Client;
use serde_json::Value;
use crate::{gcloud, helper_funcs};


pub(crate) async fn find_restaurants(client: Client) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let google_maps_api_key = gcloud::get_gmaps_api_key()?;
    // TODO: change this so it uses real location.
    let user_lat = 44.9778;
    let user_lon = -93.2650;

    let url = format!(
        "https://maps.googleapis.com/maps/api/place/nearbysearch/json?location={},{}&radius=1500&type=restaurant&key={}",
        user_lat, user_lon, google_maps_api_key
    );

    let response = client.get(&url).send().await?.json::<Value>().await?; // Parse JSON

    // Extract restaurant details
    if let Some(results) = response["results"].as_array() {
        for restaurant in results {
            let name = restaurant["name"].as_str().unwrap_or("Unknown");
            let address = restaurant["vicinity"].as_str().unwrap_or("No address available");

            // Extract restaurant's lat/lng
            let rest_lat = restaurant["geometry"]["location"]["lat"].as_f64().unwrap_or(0.0);
            let rest_lon = restaurant["geometry"]["location"]["lng"].as_f64().unwrap_or(0.0);

            // Calculate distance using Haversine formula
            let distance = helper_funcs::haversine_distance(user_lat, user_lon, rest_lat, rest_lon);

            let types = restaurant["types"]
                .as_array()
                .map(|t| t.iter().filter_map(|v| v.as_str()).collect::<Vec<_>>().join(", "))
                .unwrap_or("Unknown type".to_string());

            let price_level = restaurant["price_level"].as_i64().unwrap_or(-1); // -1 if not available

            println!(
                "Name: {}\nLocation: {}, {}\nAddress: {}\nType: {}\nPrice Level: {}\nDistance: {:.2} km\n",
                name, rest_lat, rest_lon, address, types, price_level, distance
            );
        }
    } else {
        println!("No restaurants found.");
    }

    Ok(())
}

