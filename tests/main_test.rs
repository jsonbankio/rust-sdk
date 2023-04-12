extern crate jsonbank;

use jsonbank::*;

#[derive(Debug)]
struct TestData {
    pub project: &'static str,
    pub file: &'static str,
    pub id: Option<String>,
    pub path: String,
}

// user_path - returns path for user
fn user_path(path: String) -> String {
    format!("{}/{}", JSONBANK, path)
}


// init - initialize test
fn init() -> (JsonBank, TestData) {
    let mut jsb = JsonBank::new_without_config();
    // set host to dev server
    jsb.set_host("http://localhost:2223");

    let mut data = TestData {
        project: "jsonbank/sdk-test",
        file: "index.json",
        id: None,
        path: "".to_string(),
    };

    // set path
    data.path = format!("{}/{}", data.project, data.file);

    // get metadata for test file
    let path = format!("{}/{}", data.project, data.file);
    let meta = jsb.get_document_meta(&path).unwrap();

    // update id
    data.id = Some(meta.id);

    (jsb, data)
}

#[test]
fn get_content() {
    let (jsb, data) = init();

    // get content by id
    let content = match jsb.get_content(&data.id.unwrap()) {
        Ok(content) => content,
        Err(err) => panic!("{:?}", err),
    };

    assert_eq!(content["author"], JSONBANK);

    // get content by path
    let content = match jsb.get_content(&data.path) {
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