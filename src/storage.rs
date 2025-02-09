use std::collections::HashMap;

pub type UUID = String;

#[derive(serde::Deserialize)]
pub struct CreateEvent {
    pub name: String,
    pub address: String,
    pub food_type: String,
    pub max_claims: u64,
    pub deadline: Option<u64>,
}

#[derive(serde::Serialize)]
pub struct CreateEventResponse {
    pub error: bool,
    pub error_msg: Option<String>,
    pub id: Option<UUID>,
    pub delete_id: Option<UUID>,
}

#[derive(serde::Serialize)]
pub struct GetEventResponse {
    pub error: bool,
    pub error_msg: Option<String>,

    pub name: Option<String>,
    pub food_type: Option<String>,
    pub max_claims: Option<u64>,
    pub claims: Option<u64>,
    pub post_date: Option<u64>,
    pub deadline: Option<u64>,
}

#[derive(serde::Serialize)]
pub struct ClaimEventResponse {
    pub error: bool,
    pub error_msg: Option<String>,

    pub address: Option<String>,
    pub code_url: Option<String>,
}

#[derive(serde::Serialize)]
pub struct DeleteEventResponse {
    pub error: bool,
    pub error_msg: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Event {
    pub name: String,
    pub address: String,
    pub food_type: String,
    pub max_claims: u64,
    pub claims: u64,
    pub post_date: u64,
    pub deadline: Option<u64>,
    pub delete_id: UUID,
    pub claim_id: UUID,
}

pub type Events = HashMap<UUID, Event>;

pub fn deserialize_json(data: String) -> Events {
    serde_json::de::from_str(&data).unwrap_or_default()
}

pub fn serialize_json(events: Events) -> Vec<u8> {
    serde_json::ser::to_vec(&events).unwrap()
}

pub fn add_event(event: Event, id: UUID) {
    let mut events = deserialize_json(std::fs::read_to_string("events.json").unwrap_or_default());
    events.insert(id, event);
    std::fs::write("events.json", serialize_json(events)).unwrap();
}

pub fn get_event(id: UUID) -> Option<Event> {
    let events = deserialize_json(std::fs::read_to_string("events.json").unwrap_or_default());
    events.get(&id).cloned()
}

pub fn set_event(event: Event, id: UUID) {
    let mut events = deserialize_json(std::fs::read_to_string("events.json").unwrap_or_default());
    events.insert(id, event);
    std::fs::write("events.json", serialize_json(events)).unwrap();
}

pub fn delete_event(id: UUID) {
    let mut events = deserialize_json(std::fs::read_to_string("events.json").unwrap_or_default());
    events.remove(&id);
    std::fs::write("events.json", serialize_json(events)).unwrap();
}

pub fn get_events() -> Vec<Event> {
    let events = deserialize_json(std::fs::read_to_string("events.json").unwrap_or_default());
    events.into_values().collect()
}