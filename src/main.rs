#![feature(unboxed_closures)]

use actix_web::web::Data;
use std::sync::Arc;
use tokio::sync::Mutex;
use actix_web::HttpServer;
use actix_web::App;
use crate::food_bank::find_food_shelters_route;
use crate::restaurants::find_restaurants_route;
use actix_cors::Cors;

mod storage;
mod food_bank;
mod restaurants;
mod helper_funcs;
mod gcloud;
mod events;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = gcloud::init()?;
    
    let client_data = Data::new(Arc::new(Mutex::new(client.clone())));

    let http_server = HttpServer::new(move || {
        App::new()
            .app_data(client_data.clone()) // Share client with Actix
            .wrap(Cors::permissive()) // Allow all origins for development
            .service(find_restaurants_route)
            .service(find_food_shelters_route)
            .service(events::create_event)
            .service(events::get_event)
            .service(events::claim_event)
            .service(events::get_code)
            .service(events::delete_event)
            .service(events::get_events)
    })
    .bind("0.0.0.0:3001")? // Use a different port for Actix
    .run();

    http_server.await?;

    Ok(())
}

