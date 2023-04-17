use std::collections::HashMap;
use serde_json::Value;
use crate::DocumentMeta;
use crate::structs::CreateDocumentBody;

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

// make_document_path - generate a document full path
// if the document has a folder, the folder will be prepended to the document name
pub fn make_document_path(document: &CreateDocumentBody) -> String {
    let mut folder = String::new();

    // if the document has a folder, prepend it to the document name
    if document.folder.is_some() {
        folder = format!("{}/", document.folder.as_ref().unwrap());
    }

    format!("{}/{}{}", document.project, folder, document.name)
}