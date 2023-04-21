mod functions;

use jsonbank::*;
use functions::*;


// init - initializes test
fn init() -> (JsonBank, TestData) {
    let env = load_env();

    let mut jsb = JsonBank::new_without_config();
    jsb.set_host(env.host.as_str());

    prepare_instance(jsb, false)
}

// user_path - returns path for user
fn user_path(path: String) -> String {
    format!("{}/{}", JSONBANK, path)
}


#[test]
fn get_content() {
    let (jsb, data) = init();

    // get content by id
    let content: JsonObject = match jsb.get_content(&data.id.unwrap()) {
        Ok(content) => content,
        Err(err) => panic!("{:?}", err),
    };

    assert_eq!(content["author"], JSONBANK);

    // get content by path
    let content: JsonObject = match jsb.get_content(&data.path) {
        Ok(content) => content,
        Err(err) => panic!("{:?}", err),
    };

    assert_eq!(content["author"], JSONBANK);
}

#[test]
fn get_content_as_string(){
    let (jsb, data) = init();

    // get content by id
    let content = match jsb.get_content_as_string(&data.id.unwrap()) {
        Ok(content) => content,
        Err(err) => panic!("{:?}", err),
    };

   // string contains JSONBANK
    assert!(content.contains(JSONBANK));

    // get content by path
    let content = match jsb.get_content_as_string(&data.path) {
        Ok(content) => content,
        Err(err) => panic!("{:?}", err),
    };

    // string contains JSONBANK
    assert!(content.contains(JSONBANK));
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
    assert_eq!(meta.path, data.name);

    // get metadata by path
    let meta = match jsb.get_document_meta(&data.path) {
        Ok(meta) => meta,
        Err(err) => panic!("{:?}", err),
    };

    assert_eq!(user_path(meta.project), data.project);
    assert_eq!(meta.path, data.name);
}


#[test]
fn get_github_content() {
    let (jsb, _data) = init();

    // get content by id
    let content: JsonObject = match jsb.get_github_content("jsonbankio/jsonbank-js/package.json") {
        Ok(content) => content,
        Err(err) => panic!("{:?}", err),
    };

    println!("{:?}", content);

    assert_eq!(content["name"], JSONBANK);
    assert_eq!(content["author"], JSONBANK_IO);
}

#[test]
fn get_github_content_as_string(){
    let (jsb, _data) = init();

    // get content by id
    let content = match jsb.get_github_content_as_string("jsonbankio/jsonbank-js/package.json") {
        Ok(content) => content,
        Err(err) => panic!("{:?}", err),
    };

    // string contains JSONBANK
    assert!(content.contains("prepublishOnly"));
}