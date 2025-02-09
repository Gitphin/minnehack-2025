use std::collections::HashMap;

pub type UUID = String;

#[derive(serde::Deserialize)]
pub struct CreateEvent {
    pub name: String,
    pub food_type: String,
    pub max_claims: u64,
    pub deadline: u64,
}

#[derive(serde::Serialize)]
pub struct CreateEventResponse {
    pub error: bool,
    pub error_msg: Option<String>,
    pub id: Option<UUID>,
    pub delete_id: Option<UUID>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Event {
    pub name: String,
    pub food_type: String,
    pub max_claims: u64,
    pub claims: u64,
    pub post_date: u64,
    pub deadline: u64,
    pub delete_id: UUID,
}

pub type Events = HashMap<UUID, Event>;

pub fn deserialize_json(data: String) -> Events {
    serde_json::de::from_str(&data).unwrap()
}

pub fn serialize_json(events: Events) -> Vec<u8> {
    serde_json::ser::to_vec(&events).unwrap()
}

pub fn add_event(event: Event, id: UUID) {
    let mut events = deserialize_json(std::fs::read_to_string("events.json").unwrap());
    events.insert(id, event);
    std::fs::write("events.json", serialize_json(events)).unwrap();
}