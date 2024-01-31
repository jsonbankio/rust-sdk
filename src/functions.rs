use crate::{ContentSize, DocumentMeta, JsonObject, JsonValue};
use crate::structs::{CreateDocumentBody, CreateFolderBody, Folder, FolderStats};

/// Converts a HashMap to a DocumentMeta struct
pub fn json_object_to_document_meta(map: &JsonObject) -> DocumentMeta {
    let size = map["contentSize"].as_object().unwrap();
    // get optional folderId
    let folder_id = if map.contains_key("folderId") {
        Some(map["folderId"].as_str().unwrap().to_string())
    } else {
        None
    };

    DocumentMeta {
        id: map["id"].as_str().unwrap().to_string(),
        project: map["project"].as_str().unwrap().to_string(),
        name: map["name"].as_str().unwrap().to_string(),
        path: map["path"].as_str().unwrap().to_string(),
        content_size: ContentSize {
            number: size["number"].as_u64().unwrap(),
            string: size["string"].as_str().unwrap().to_string(),
        },
        folder_id,
        updated_at: map["updatedAt"].as_str().unwrap().to_string(),
        created_at: map["createdAt"].as_str().unwrap().to_string(),
    }
}

/// Converts a HashMap to a Folder
pub fn json_object_to_folder(map: &JsonObject) -> Folder {
    // if stats exists, convert it to a FolderStats
    let stats = if map.contains_key("stats") {
        let stats_map = map["stats"].as_object().unwrap();
        Some(FolderStats {
            documents: stats_map["documents"].as_i64().unwrap() as i32,
            folders: stats_map["folders"].as_i64().unwrap() as i32,
        })
    } else {
        None
    };

    Folder {
        id: map["id"].as_str().unwrap().to_string(),
        name: map["name"].as_str().unwrap().to_string(),
        path: map["path"].as_str().unwrap().to_string(),
        project: map["project"].as_str().unwrap().to_string(),
        created_at: map["createdAt"].as_str().unwrap().to_string(),
        updated_at: map["updatedAt"].as_str().unwrap().to_string(),
        stats
    }
}

/// checks if a string is valid JSON
pub fn is_valid_json(json: &str) -> bool {
    serde_json::from_str::<JsonValue>(json).is_ok()
}

/// Generate a document full path.
/// If the document has a folder, the folder will be prepended to the document name
pub fn make_document_path(document: &CreateDocumentBody) -> String {
    let mut folder = String::new();

    // if the document has a folder, prepend it to the document name
    if document.folder.is_some() {
        folder = format!("{}/", document.folder.as_ref().unwrap());
    }

    format!("{}/{}{}", document.project, folder, document.name)
}

/// Generate a folder full path.
/// If the folder has a parent folder, the parent folder will be prepended to the folder name
pub fn make_folder_path(folder: &CreateFolderBody) -> String {
    let mut parent_folder = String::new();

    // if the folder has a parent folder, prepend it to the folder name
    if folder.folder.is_some() {
        parent_folder = format!("{}/", folder.folder.as_ref().unwrap());
    }

    format!("{}/{}{}", folder.project, parent_folder, folder.name)
}