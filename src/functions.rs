use crate::{ContentSize, DocumentMeta, JsonObject, JsonValue};
use crate::structs::{
    CreateDocumentBody, CreateFolderBody, Folder, FolderStats, ListDocumentsResponse, ListFoldersResponse,
    ListParams, ListedFolder, ListedProject, PaginatedDocuments, PaginatedFolders, PaginationMeta,
    ScanProjectParams, ScanProjectResponse,
};

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
// ====== Listing ======

/// Converts a json value to a JsonObject, empty if it is not an object
fn json_value_to_object(value: &JsonValue) -> JsonObject {
    match value.as_object() {
        Some(map) => map.clone().into_iter().collect(),
        None => JsonObject::new(),
    }
}

/// Reads a nested object from a map, empty if missing
fn object_at(map: &JsonObject, key: &str) -> JsonObject {
    match map.get(key) {
        Some(value) => json_value_to_object(value),
        None => JsonObject::new(),
    }
}

/// Reads a string from a map, empty if missing
fn string_at(map: &JsonObject, key: &str) -> String {
    match map.get(key).and_then(|value| value.as_str()) {
        Some(value) => value.to_string(),
        None => String::new(),
    }
}

/// Reads a number from a map, zero if missing
fn number_at(map: &JsonObject, key: &str) -> i32 {
    match map.get(key).and_then(|value| value.as_i64()) {
        Some(value) => value as i32,
        None => 0,
    }
}

/// Converts a HashMap to a PaginationMeta
fn json_object_to_pagination_meta(map: &JsonObject) -> PaginationMeta {
    PaginationMeta {
        page: number_at(map, "page"),
        per_page: number_at(map, "perPage"),
        total: number_at(map, "total"),
        last_page: number_at(map, "lastPage"),
    }
}

/// Converts a HashMap to a PaginatedDocuments
fn json_object_to_paginated_documents(map: &JsonObject) -> PaginatedDocuments {
    let data = match map.get("data").and_then(|value| value.as_array()) {
        Some(items) => items
            .iter()
            .map(|item| json_object_to_document_meta(&json_value_to_object(item)))
            .collect(),
        None => Vec::new(),
    };

    PaginatedDocuments {
        data,
        meta: json_object_to_pagination_meta(&object_at(map, "meta")),
    }
}

/// Converts a HashMap to a PaginatedFolders
fn json_object_to_paginated_folders(map: &JsonObject) -> PaginatedFolders {
    let data = match map.get("data").and_then(|value| value.as_array()) {
        Some(items) => items
            .iter()
            .map(|item| json_object_to_folder(&json_value_to_object(item)))
            .collect(),
        None => Vec::new(),
    };

    PaginatedFolders {
        data,
        meta: json_object_to_pagination_meta(&object_at(map, "meta")),
    }
}

/// Converts a HashMap to a ListedProject
fn json_object_to_listed_project(map: &JsonObject) -> ListedProject {
    ListedProject {
        slug: string_at(map, "slug"),
        title: string_at(map, "title"),
        access: string_at(map, "access"),
    }
}

/// Reads the listed folder, which the api leaves out when the project root is listed
fn json_object_to_listed_folder(map: &JsonObject) -> Option<ListedFolder> {
    let folder = match map.get("folder") {
        Some(value) if value.is_object() => json_value_to_object(value),
        _ => return None,
    };

    // parent_folder is only set when the listed folder is nested
    let parent_folder = if folder.contains_key("parentFolder") {
        Some(string_at(&folder, "parentFolder"))
    } else {
        None
    };

    Some(ListedFolder {
        id: string_at(&folder, "id"),
        name: string_at(&folder, "name"),
        path: string_at(&folder, "path"),
        parent_folder,
    })
}

/// Converts a HashMap to a ScanProjectResponse
pub fn json_object_to_scan_project(map: &JsonObject) -> ScanProjectResponse {
    ScanProjectResponse {
        project: json_object_to_listed_project(&object_at(map, "project")),
        folder: json_object_to_listed_folder(map),
        documents: json_object_to_paginated_documents(&object_at(map, "documents")),
        folders: json_object_to_paginated_folders(&object_at(map, "folders")),
    }
}

/// Converts a HashMap to a ListDocumentsResponse
pub fn json_object_to_list_documents(map: &JsonObject) -> ListDocumentsResponse {
    ListDocumentsResponse {
        project: json_object_to_listed_project(&object_at(map, "project")),
        folder: json_object_to_listed_folder(map),
        documents: json_object_to_paginated_documents(&object_at(map, "documents")),
    }
}

/// Converts a HashMap to a ListFoldersResponse
pub fn json_object_to_list_folders(map: &JsonObject) -> ListFoldersResponse {
    ListFoldersResponse {
        project: json_object_to_listed_project(&object_at(map, "project")),
        folder: json_object_to_listed_folder(map),
        folders: json_object_to_paginated_folders(&object_at(map, "folders")),
    }
}

/// Adds a query param, skipping unset values
fn insert_query_string(query: &mut JsonObject, key: &str, value: &Option<String>) {
    if let Some(value) = value {
        query.insert(key.to_string(), JsonValue::String(value.to_string()));
    }
}

/// Adds a query param, skipping unset values
fn insert_query_number(query: &mut JsonObject, key: &str, value: &Option<i32>) {
    if let Some(value) = value {
        query.insert(key.to_string(), JsonValue::from(*value));
    }
}

/// Builds the query of the `scan_project` function
pub fn scan_project_params_to_query(params: &ScanProjectParams) -> Option<JsonObject> {
    let mut query = JsonObject::new();

    insert_query_string(&mut query, "folder", &params.folder);
    insert_query_number(&mut query, "documentsPage", &params.documents_page);
    insert_query_number(&mut query, "documentsPerPage", &params.documents_per_page);
    insert_query_number(&mut query, "foldersPage", &params.folders_page);
    insert_query_number(&mut query, "foldersPerPage", &params.folders_per_page);
    insert_query_string(&mut query, "sort", &params.sort);
    insert_query_string(&mut query, "order", &params.order);

    if query.is_empty() { None } else { Some(query) }
}

/// Builds the query of the `list_documents` and `list_folders` functions
pub fn list_params_to_query(params: &ListParams) -> Option<JsonObject> {
    let mut query = JsonObject::new();

    insert_query_string(&mut query, "folder", &params.folder);
    insert_query_number(&mut query, "page", &params.page);
    insert_query_number(&mut query, "perPage", &params.per_page);
    insert_query_string(&mut query, "sort", &params.sort);
    insert_query_string(&mut query, "order", &params.order);

    if query.is_empty() { None } else { Some(query) }
}
