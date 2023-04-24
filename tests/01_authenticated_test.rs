extern crate dotenv;

mod functions;

use std::time::{SystemTime, UNIX_EPOCH};
use jsonbank::{JsonBank, InitConfig, Keys, JsonObject, JSONBANK};
use functions::*;
use jsonbank::structs::{CreateDocumentBody, CreateFolderBody, Folder, UploadDocumentBody};

// test_file_content - returns test file content
fn test_file_content() -> String {
    r#"{
        "name": "JsonBank SDK Test File",
        "author": "jsonbank"
    }"#.to_string()
}

// init - initializes test
fn init() -> (JsonBank, TestData) {
    let env = load_env();
    let config = InitConfig {
        host: Some(env.host.clone()),
        keys: Some(Keys {
            public: Some(env.public_key),
            private: Some(env.private_key),
        }),
    };

    let mut jsb = JsonBank::new(config);
    jsb.set_host(env.host.as_str());

    prepare_instance(jsb, true)
}


#[test]
fn authenticate() {
    let (mut jsb, _data) = init();

    let auth = match jsb.authenticate() {
        Ok(auth) => auth,
        Err(err) => panic!("{:?}", err),
    };

    assert_eq!(auth.authenticated, true);

    // test is_authenticated since we are authenticated
    assert_eq!(jsb.is_authenticated(), true);


    // test get_username since we are authenticated
    let username = match jsb.get_username() {
        Ok(username) => username,
        Err(err) => panic!("{:?}", err),
    };

    assert_eq!(username, JSONBANK);
}


#[test]
fn has_own_document(){
    let (jsb, data) = init();

    // check if document exists by id
    let has_id = jsb.has_own_document(&data.id.unwrap());

    assert_eq!(has_id, true);

    // check if document exists by path
    let has_path = jsb.has_own_document(&data.path);

    assert_eq!(has_path, true);
}


#[test]
fn get_own_content() {
    let (jsb, data) = init();

    // get content by id
    let content: JsonObject = match jsb.get_own_content(&data.id.unwrap()) {
        Ok(content) => content,
        Err(err) => panic!("{:?}", err),
    };

    assert_eq!(content["author"], JSONBANK);

    // get content by path
    let content: JsonObject = match jsb.get_own_content(&data.path) {
        Ok(content) => content,
        Err(err) => panic!("{:?}", err),
    };

    assert_eq!(content["author"], JSONBANK);
}


#[test]
fn get_own_document_meta() {
    let (jsb, data) = init();

    // get metadata by id
    let meta = match jsb.get_own_document_meta(&data.id.unwrap()) {
        Ok(meta) => meta,
        Err(err) => panic!("{:?}", err),
    };

    assert_eq!(meta.project, data.project);
    assert_eq!(meta.path, data.name);

    // get metadata by path
    let meta = match jsb.get_own_document_meta(&data.path) {
        Ok(meta) => meta,
        Err(err) => panic!("{:?}", err),
    };

    assert_eq!(meta.project, data.project);
    assert_eq!(meta.path, data.name);
}



#[test]
fn create_document() {
    let (jsb, data) = init();

    // delete test file if it exists
    let res = match jsb.delete_document(&data.path) {
        Ok(res) => res,
        Err(err) => panic!("{:?}", err),
    };

    assert_eq!(res.deleted, true);

    let content = CreateDocumentBody {
        name: data.name,
        project: data.project.clone(),
        folder: None,
        content: test_file_content(),
    };

    let new_doc = match jsb.create_document(content) {
        Ok(doc) => doc,
        Err(err) => panic!("{:?}", err),
    };

    assert_eq!(new_doc.project, data.project);
}


#[test]
fn create_document_if_not_exists() {
    let (jsb, data) = init();

    let content = CreateDocumentBody {
        name: data.name,
        project: data.project.clone(),
        folder: None,
        content: test_file_content(),
    };

    let new_doc = match jsb.create_document_if_not_exists(content) {
        Ok(doc) => doc,
        Err(err) => panic!("{:?}", err),
    };

    println!("{:?}", new_doc);

    assert_eq!(new_doc.project, data.project);
}

#[test]
fn upload_document() {
    let (jsb, data) = init();

    // delete test file if it exists
    let _ = match jsb.delete_document(format!("{}/{}", data.project, "upload.json").as_str()) {
        Ok(res) => res,
        Err(err) => panic!("{:?}", err),
    };


    let file_path = "tests/upload.json";
    let new_doc = match jsb.upload_document(UploadDocumentBody {
        file_path: file_path.to_string(),
        project: data.project.clone(),
        name: None,
        folder: None,
    }) {
        Ok(doc) => doc,
        Err(err) => panic!("{:?}", err),
    };

    assert_eq!(new_doc.project, data.project);
    // both paths should be the same
    assert_eq!(new_doc.path, "upload.json");
}

#[test]
fn update_own_document() {
    let (jsb, data) = init();

    // get current timestamp
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();


    let content = r#"{
    		"name": "JsonBank SDK Test File",
    		"author": "jsonbank",
			"updated": true,
			"timestamp": "#.to_string() + &timestamp.to_string() + r#"
		}"#;


    let res = match jsb.update_own_document(&data.path, content.to_string()) {
        Ok(res) => res,
        Err(err) => panic!("{:?}", err),
    };

    // changed must be true
    assert_eq!(res.changed, true);

    // revert the changes
    let _ = match jsb.update_own_document(&data.path, test_file_content()) {
        Ok(res) => res,
        Err(err) => panic!("{:?}", err),
    };
}

#[test]
fn create_folder() {
    let (jsb, data) = init();

    let res = match jsb.create_folder(CreateFolderBody {
        name: "folder".to_string(),
        project: data.project.clone(),
        folder: None,
    }) {
        Ok(res) => res,
        Err(err) => {
            // if error code is `name.exists` then the folder already exists and we can continue without throwing an error
            if err.code == "name.exists" {
                // log the error but still return a NewFolder struct
                eprintln!("Expected Error: {:?}", err);

                Folder {
                    id: "".to_string(),
                    name: "folder".to_string(),
                    project: data.project.clone(),
                    created_at: "".to_string(),
                    path: "folder".to_string(),
                    updated_at: "".to_string(),
                    stats: None
                }
            } else {
                panic!("{:?}", err);
            }
        }
    };

    if res.name != "folder" || res.project != data.project {
        panic!("New folder data mismatch");
    }
}

#[test]
fn upload_document_to_folder() {
    let (jsb, data) = init();

    // delete test file if it exists
    let _ = jsb.delete_document(format!("{}/{}", data.project, "folder/upload.json").as_str());

    let file_path = "tests/upload.json";
    let new_doc = match jsb.upload_document(UploadDocumentBody {
        file_path: file_path.to_string(),
        project: data.project.clone(),
        name: None,
        folder: Some("folder".to_string()),
    }) {
        Ok(doc) => doc,
        Err(err) => panic!("{:?}", err),
    };

    assert_eq!(new_doc.project, data.project);
    // both paths should be the same
    assert_eq!(new_doc.path, "folder/upload.json");
}

#[test]
fn get_folder() {
    let (jsb, data) = init();

    let f = format!("{}/{}", data.project, "folder");

    let folder = match jsb.get_folder(f.as_str()) {
        Ok(folder) => folder,
        Err(err) => panic!("{:?}", err),
    };

    assert_eq!(folder.name, "folder");
    assert_eq!(folder.project, data.project);

    // check if it works with id
    let folder = match jsb.get_folder(folder.id.as_str()) {
        Ok(folder) => folder,
        Err(err) => panic!("{:?}", err),
    };

    assert_eq!(folder.name, "folder");
    assert_eq!(folder.project, data.project);
}

#[test]
fn get_folder_with_stats(){
    let (jsb, data) = init();

    let f = format!("{}/{}", data.project, "folder");

    let folder = match jsb.get_folder_with_stats(f.as_str()) {
        Ok(folder) => folder,
        Err(err) => panic!("{:?}", err),
    };

    println!("{:?}", folder);

    assert_eq!(folder.name, "folder");
    assert_eq!(folder.project, data.project);
    assert_eq!(folder.stats.is_some(), true);

    // check if it works with id
    let folder = match jsb.get_folder_with_stats(folder.id.as_str()) {
        Ok(folder) => folder,
        Err(err) => panic!("{:?}", err),
    };

    assert_eq!(folder.name, "folder");
    assert_eq!(folder.project, data.project);
    assert_eq!(folder.stats.is_some(), true);
}

#[test]
fn create_folder_if_not_exists() {
    let (jsb, data) = init();

    let body = CreateFolderBody {
        name: "folder".to_string(),
        project: data.project.clone(),
        folder: None,
    };

    let res = match jsb.create_folder_if_not_exists(body) {
        Ok(res) => res.0,
        Err(err) => panic!("{:?}", err),
    };

    if res.name != "folder" || res.project != data.project {
        panic!("New folder data mismatch");
    }
}