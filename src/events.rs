use actix_web::{get, post, web, HttpResponse, Responder};

use crate::storage::{add_event, CreateEvent, CreateEventResponse, Event, GetEventResponse};

use uuid::Uuid;

#[get("/events/{id}")]
async fn get_event(path: web::Path<(String,)>) -> impl Responder {
    let id = path.into_inner().0;

    let event = crate::storage::get_event(id);

    if event.is_none() {
        return HttpResponse::NotFound().json(GetEventResponse {
            error: true,
            error_msg: Some("Could not find event".to_owned()),
            name: None,
            food_type: None,
            max_claims: None,
            claims: None,
            post_date: None,
            deadline: None
        });
    }

    let event = event.unwrap();
    
    HttpResponse::Ok().json(GetEventResponse {
        error: false,
        error_msg: None,
        name: Some(event.name),
        food_type: Some(event.food_type),
        max_claims: Some(event.max_claims),
        claims: Some(event.claims),
        post_date: Some(event.post_date),
        deadline: Some(event.deadline)
    })
}

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
        address: createevent.address,
        food_type: createevent.food_type,
        max_claims: createevent.max_claims,
        claims: 0,
        post_date: time,
        deadline: createevent.deadline,
        delete_id: delete_id.clone()
    };
    
    add_event(event, id.clone());

    HttpResponse::Ok().json(CreateEventResponse {
        error: false,
        error_msg: None,
        id: Some(id),
        delete_id: Some(delete_id),
    })
}