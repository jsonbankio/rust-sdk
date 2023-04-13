mod functions;

use std::collections::HashMap;
use serde_json::{Value};
use jsonbank::*;
use functions::*;


// init - initializes test
fn init() -> (JsonBank, TestData) {
    let jsb = JsonBank::new_without_config();
    prepare_instance(jsb, false)
}


#[test]
fn get_content() {
    let (jsb, data) = init();

    // get content by id
    let content: HashMap<String, Value> = match jsb.get_content(&data.id.unwrap()) {
        Ok(content) => content,
        Err(err) => panic!("{:?}", err),
    };

    assert_eq!(content["author"], JSONBANK);

    // get content by path
    let content: HashMap<String, Value> = match jsb.get_content(&data.path) {
        Ok(content) => content,
        Err(err) => panic!("{:?}", err),
    };

    assert_eq!(content["author"], JSONBANK);
}


#[test]
fn get_document_meta() {
    let (jsb, data) = init();

    // get metadata by id
    let meta = match jsb.get_document_meta(&data.id.unwrap()) {
        Ok(meta) => meta,
        Err(err) => panic!("{:?}", err),
    };

    assert_eq!(user_path(meta.project), data.project);
    assert_eq!(meta.path, data.file);

    // get metadata by path
    let meta = match jsb.get_document_meta(&data.path) {
        Ok(meta) => meta,
        Err(err) => panic!("{:?}", err),
    };

    assert_eq!(user_path(meta.project), data.project);
    assert_eq!(meta.path, data.file);
}


#[test]
fn get_github_content() {
    let (jsb, _data) = init();

    // get content by id
    let content: HashMap<String, Value> = match jsb.get_github_content("jsonbankio/jsonbank-js/package.json") {
        Ok(content) => content,
        Err(err) => panic!("{:?}", err),
    };

    println!("{:?}", content);

    assert_eq!(content["name"], JSONBANK);
    assert_eq!(content["author"], JSONBANKIO);
}