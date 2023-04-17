use std::collections::HashMap;
use serde_json::Value;
use crate::DocumentMeta;

// hash_map_to_document_meta - converts a HashMap to a DocumentMeta
pub fn hash_map_to_document_meta(map: &HashMap<String, Value>) -> DocumentMeta {
    DocumentMeta {
        id: map["id"].as_str().unwrap().to_string(),
        project: map["project"].as_str().unwrap().to_string(),
        path: map["path"].as_str().unwrap().to_string(),
        updated_at: map["updatedAt"].as_str().unwrap().to_string(),
        created_at: map["createdAt"].as_str().unwrap().to_string(),
    }
}

// is_valid_json - checks if a string is valid JSON
pub fn is_valid_json(json: &str) -> bool {
    serde_json::from_str::<Value>(json).is_ok()
}