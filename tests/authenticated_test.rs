extern crate dotenv;

mod functions;

use std::collections::HashMap;
use serde_json::{Value};
use jsonbank::*;
use functions::*;


struct Env {
    host: String,
    public_key: String,
    private_key: String,
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

    let jsb = JsonBank::new(InitConfig {
        host: Some(env.host),
        keys: Some(Keys {
            public: Some(env.public_key),
            private: Some(env.private_key),
        }),
    });

    prepare_instance(jsb, "sdk-test".to_string())
}


#[test]
fn get_own_content() {
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