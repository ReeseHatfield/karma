use serde::Deserialize;
use serde_json::Value;

pub struct WikiGraphEntity {
    pub r#type: String,
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct Object {
    pub r#type: String,
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct WikiDataTriple {
    pub subject: Object,
    pub predicate: Object,
    pub object: Object,
}

#[derive(Debug, Clone)]
pub struct TopicPath {
    pub from_subject: String,
    pub via_predicate: String,
    pub to_object_label: String,
    pub to_object_id: String,
}
