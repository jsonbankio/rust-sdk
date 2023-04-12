use jsonbank::{JSONBANK, JsonBank};

#[derive(Debug)]
pub struct TestData {
    pub project:String,
    pub file: String,
    pub id: Option<String>,
    pub path: String,
}

// user_path - returns path for user
pub fn user_path(path: String) -> String {
    format!("{}/{}", JSONBANK, path)
}


// prepare_instance - prepares instance for testing
pub fn prepare_instance(mut jsb: JsonBank, project: String) -> (JsonBank, TestData) {
    // set host to dev server
    jsb.set_host("http://localhost:2223");



    let mut data = TestData {
        project,
        file: "index.json".to_string(),
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