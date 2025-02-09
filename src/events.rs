use actix_web::{dev::HttpServiceFactory, get, post, web, HttpResponse, Responder};

use crate::storage::{CreateEvent, CreateEventResponse, Event};

use uuid::Uuid;

#[post("/events/create")]
async fn create_event(data: web::Json<CreateEvent>) -> impl Responder {
    let start = std::time::SystemTime::now();
    let time = start
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let createevent = data.into_inner();

    let id = Uuid::new_v4().as_hyphenated().to_string();
    let delete_id = Uuid::new_v4().as_hyphenated().to_string();

    let event = Event {
        name: createevent.name,
        food_type: createevent.food_type,
        max_claims: createevent.max_claims,
        claims: 0,
        post_date: time,
        deadline: createevent.deadline,
        delete_id: delete_id.clone()
    };

    HttpResponse::Ok().json(CreateEventResponse {
        error: false,
        error_msg: None,
        id: Some(id),
        delete_id: Some(delete_id),
    })
}

pub fn event_routes() -> Vec<impl HttpServiceFactory> {
    vec![
        create_event
    ]
}