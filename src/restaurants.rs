use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use reqwest::Client;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::{gcloud};

// Fetch restaurants using Google Places API
pub(crate) async fn find_restaurants(client: Client) -> Result<Vec<Value>, Box<dyn std::error::Error + Send + Sync>> {
    let google_maps_api_key = gcloud::get_gmaps_api_key()?;
    
    let user_lat = 44.9778;
    let user_lon = -93.2650;

    let url = format!(
        "https://maps.googleapis.com/maps/api/place/nearbysearch/json?location={},{}&radius=2500&type=restaurant&key={}",
        user_lat, user_lon, google_maps_api_key
    );

    let response = client.get(&url).send().await?.json::<Value>().await?;

    if let Some(results) = response["results"].as_array() {
        return Ok(results.clone());
    } else {
        return Err("No restaurants found.".into());
    }
}

// Actix Web Route to Fetch Nearby Restaurants
#[get("/find_restaurants")]
async fn find_restaurants_route(client: web::Data<Arc<Mutex<Client>>>) -> impl Responder {
    let client = client.lock().await;
    
    match find_restaurants(client.clone()).await {
        Ok(restaurants) => HttpResponse::Ok().json(restaurants),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

// Configure Actix Web Services
// pub fn config(cfg: &mut web::ServiceConfig) {
//     cfg.service(find_restaurants_route);
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = Arc::new(Mutex::new(Client::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            // .configure(config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

