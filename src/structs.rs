// AuthenticatedKey struct - Authenticated key
#[derive(Debug)]
pub struct AuthenticatedKey {
    pub title: String,
    pub projects: Vec<String>,
}

// AuthenticatedData struct - Authenticated data
#[derive(Debug)]
pub struct AuthenticatedData {
    pub authenticated: bool,
    pub username: String,
    pub api_key: AuthenticatedKey,
}

// impl clone for authenticated data
impl Clone for AuthenticatedData {
    fn clone(&self) -> Self {
        AuthenticatedData {
            authenticated: self.authenticated,
            username: self.username.to_string(),
            api_key: AuthenticatedKey {
                title: self.api_key.title.to_string(),
                projects: self.api_key.projects.clone(),
            },
        }
    }
}


// CreateDocumentBody struct - Create document body
#[derive(Debug)]
pub struct CreateDocumentBody {
    pub name: String,
    pub project: String,
    pub folder: Option<String>,
    pub content: String,
}

// NewDocument struct - New document
#[derive(Debug)]
pub struct NewDocument {
    pub id: String,
    pub name: String,
    pub path: String,
    pub project: String,
    pub created_at: String,
    // this field is not returned by the api
    // it is populated by the `create_document_if_not_exists` function
    pub exists: bool,
}

// DeletedDocument struct - Deleted document
#[derive(Debug)]
pub struct DeletedDocument {
    pub deleted: bool,
}