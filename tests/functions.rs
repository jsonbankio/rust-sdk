use jsonbank::{JsonBank};

#[derive(Debug)]
pub struct TestData {
    pub project: String,
    pub name: String,
    pub id: Option<String>,
    pub path: String,
}

// prepare_instance - prepares instance for testing
pub fn prepare_instance(jsb: JsonBank, authenticated: bool) -> (JsonBank, TestData) {
    let project = if authenticated {
        // no username required for authenticated user
        "sdk-test".to_string()
    } else {
        // username is required for public access
        "jsonbank/sdk-test".to_string()
    };


    let mut data = TestData {
        project,
        name: "index.json".to_string(),
        id: None,
        path: "".to_string(),
    };

    // set path
    data.path = format!("{}/{}", data.project, data.name);

    let meta = if authenticated {
        jsb.get_own_document_meta(&data.path).unwrap()
    } else {
        jsb.get_document_meta(&data.path).unwrap()
    };

    // update id
    data.id = Some(meta.id);

    (jsb, data)
}
