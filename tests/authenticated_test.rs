extern crate dotenv;

mod functions;

use std::collections::HashMap;
use serde_json::{Value};
use jsonbank::*;
use functions::*;
use jsonbank::structs::{CreateDocumentBody, UploadDocumentBody};


#[derive(Debug)]
struct Env {
    host: String,
    public_key: String,
    private_key: String,
}

// test_file_content - returns test file content
fn test_file_content() -> String {
    r#"{
        "name": "JsonBank SDK Test File",
        "author": "jsonbank"
    }"#.to_string()
}

// load env
// this function loads the public and private keys from the environment file
// at the root of the project
fn load_env() -> Env {
    dotenv::dotenv().ok();
    Env {
        host: std::env::var("JSB_HOST").unwrap(),
        public_key: std::env::var("JSB_PUBLIC_KEY").unwrap(),
        private_key: std::env::var("JSB_PRIVATE_KEY").unwrap(),
    }
}


// init - initializes test
fn init() -> (JsonBank, TestData) {
    let env = load_env();
    let config = InitConfig {
        host: Some(env.host),
        keys: Some(Keys {
            public: Some(env.public_key),
            private: Some(env.private_key),
        }),
    };

    let jsb = JsonBank::new(config);

    prepare_instance(jsb, true)
}


#[test]
fn get_own_content() {
    let (jsb, data) = init();

    // get content by id
    let content: HashMap<String, Value> = match jsb.get_own_content(&data.id.unwrap()) {
        Ok(content) => content,
        Err(err) => panic!("{:?}", err),
    };

    assert_eq!(content["author"], JSONBANK);

    // get content by path
    let content: HashMap<String, Value> = match jsb.get_own_content(&data.path) {
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
    let new_doc = match jsb.upload_document(UploadDocumentBody{
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
fn upload_document_to_folder() {
    let (jsb, data) = init();

    // delete test file if it exists
    let _ = jsb.delete_document(format!("{}/{}", data.project, "folder/upload.json").as_str());

    let file_path = "tests/upload.json";
    let new_doc = match jsb.upload_document(UploadDocumentBody{
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