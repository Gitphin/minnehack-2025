use actix_web::{get, post, delete, web, HttpResponse, Responder};

use crate::storage::{add_event, ClaimEventResponse, CreateEvent, CreateEventResponse, DeleteEventResponse, Event, GetEventResponse};

use image::Luma;

use uuid::Uuid;

#[get("/events")]
async fn get_events() -> impl Responder {
    let events = crate::storage::get_events();
    return HttpResponse::Ok().json(events);
}

#[delete("/events/{id}/{delete_id}")]
async fn delete_event(path: web::Path<(String,String)>) -> impl Responder {
    let id = path.clone().0;
    let delete_id = path.into_inner().1;

    let event = crate::storage::get_event(id.clone());

    if event.is_none() {
        return HttpResponse::NotFound().json(DeleteEventResponse {
            error: true,
            error_msg: Some("Could not find event".to_owned())
        });
    }

    let event = event.unwrap();

    if event.delete_id != delete_id {
        return HttpResponse::NotFound().json(DeleteEventResponse {
            error: true,
            error_msg: Some("Invalid delete ID".to_owned())
        });
    }

    crate::storage::delete_event(id);

    return HttpResponse::Ok().json(DeleteEventResponse {
        error: false,
        error_msg: None
    })
}

#[get("/codes/{id}")]
async fn get_code(path: web::Path<(String,)>) -> impl Responder {
    let code_id = path.into_inner().0;

    let code = qrcode::QrCode::new(code_id).unwrap();

    let image = code.render::<Luma<u8>>().build();
    let _ = std::fs::remove_file("qrcode.png");
    image.save_with_format("qrcode.png", image::ImageFormat::Png).unwrap();

    return HttpResponse::Ok().content_type("image/png").body(std::fs::read("qrcode.png").unwrap())
}

#[delete("/events/{id}/claim")]
async fn claim_event(path: web::Path<(String,)>) -> impl Responder {
    let id = path.into_inner().0;

    let event = crate::storage::get_event(id.clone());

    if event.is_none() {
        return HttpResponse::NotFound().json(ClaimEventResponse {
            error: true,
            error_msg: Some("Could not find event".to_owned()),
            address: None,
            code_url: None
        });
    }

    let mut event = event.unwrap();

    if event.claims >= event.max_claims {
        return HttpResponse::NotFound().json(ClaimEventResponse {
            error: true,
            error_msg: Some("Too many claims".to_owned()),
            address: None,
            code_url: None
        });
    }

    event.claims += 1;

    crate::storage::set_event(event.clone(), id);

    return HttpResponse::Ok().json(ClaimEventResponse {
        error: false,
        error_msg: None,
        
        address: Some(event.address),
        code_url: Some("/codes/".to_owned()+&event.claim_id),
    })
}

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
        deadline: event.deadline
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
    let claim_id = Uuid::new_v4().as_hyphenated().to_string();

    let event = Event {
        name: createevent.name,
        address: createevent.address,
        food_type: createevent.food_type,
        max_claims: createevent.max_claims,
        claims: 0,
        post_date: time,
        deadline: createevent.deadline,
        delete_id: delete_id.clone(),
        claim_id
    };
    
    add_event(event, id.clone());

    HttpResponse::Ok().json(CreateEventResponse {
        error: false,
        error_msg: None,
        id: Some(id),
        delete_id: Some(delete_id),
    })
}
