use reqwest::Client;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Arc;
use tokio::sync::Mutex;  // Use `tokio::sync::Mutex` for async compatibility
use serde_json::Value;
use crate::gcloud;

pub(crate) async fn find_food_shelters(client: Client) -> Result<Vec<Value>, Box<dyn std::error::Error + Send + Sync>> {
    let google_maps_api_key = gcloud::get_gmaps_api_key()?;

    let user_lat = 44.9778;
    let user_lon = -93.2650;

    let url = format!(
        "https://maps.googleapis.com/maps/api/place/nearbysearch/json?location={},{}&radius=2500&keyword=food%20bank|soup%20kitchen|food%20pantry&key={}",
        user_lat, user_lon, google_maps_api_key
    );

    let response = client.get(&url).send().await?.json::<Value>().await?;

    if let Some(results) = response["results"].as_array() {
        return Ok(results.clone()); // Return list of food shelters
    } else {
        return Err("No food shelters found.".into());
    }
}

#[get("/find_food_shelters")]
async fn find_food_shelters_route(client: web::Data<Arc<Mutex<Client>>>) -> impl Responder {
    let client = client.lock().await;
    
    match find_food_shelters(client.clone()).await {
        Ok(shelters) => HttpResponse::Ok().json(shelters), // Return shelters as JSON
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}


// pub fn config(cfg: &mut web::ServiceConfig) {
//     cfg.service(find_food_shelters_route);
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let client = Arc::new(Mutex::new(Client::new()));

//     HttpServer::new(move || {
//         App::new()
//             .app_data(web::Data::new(client.clone()))
//             // .configure(config)
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }
