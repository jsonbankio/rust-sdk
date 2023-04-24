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

// impl clone for authenticated data.
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


// ContentSize struct - Content size
#[derive(Debug)]
pub struct ContentSize {
    pub number: u64,
    pub string: String,
}

// DocumentMeta struct - Document meta
#[derive(Debug)]
pub struct DocumentMeta {
    pub id: String,
    pub project: String,
    pub path: String,
    pub content_size: ContentSize,
    pub updated_at: String,
    pub created_at: String,
}


// CreateDocumentBody struct - Create document body
#[derive(Debug)]
pub struct CreateDocumentBody {
    pub name: String,
    pub project: String,
    pub content: String,
    pub folder: Option<String>,
}

// CreateFolderBody struct - Create folder body
#[derive(Debug)]
pub struct CreateFolderBody {
    pub name: String,
    pub project: String,
    pub folder: Option<String>,
}

// Impl clone for create folder body
impl Clone for CreateFolderBody {
    fn clone(&self) -> Self {
        CreateFolderBody {
            name: self.name.to_string(),
            project: self.project.to_string(),
            folder: self.folder.clone(),
        }
    }
}

// UploadDocumentBody struct - Upload document body
#[derive(Debug)]
pub struct UploadDocumentBody {
    pub file_path: String,
    pub project: String,
    pub name: Option<String>,
    pub folder: Option<String>,
}

// impl clone for create document body
impl Clone for CreateDocumentBody {
    fn clone(&self) -> Self {
        CreateDocumentBody {
            name: self.name.to_string(),
            project: self.project.to_string(),
            folder: self.folder.clone(),
            content: self.content.to_string(),
        }
    }
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

// FolderStats struct - Folder stats
#[derive(Debug)]
pub struct FolderStats {
    pub documents: i32,
    pub folders: i32,
}

// Folder struct - New folder
#[derive(Debug)]
pub struct Folder {
    pub id: String,
    pub name: String,
    pub path: String,
    pub project: String,
    pub created_at: String,
    pub updated_at: String,
    pub stats: Option<FolderStats>
}

// DeletedDocument struct - Deleted document
#[derive(Debug)]
pub struct DeletedDocument {
    pub deleted: bool,
}

// UpdatedDocument struct - Updated document
#[derive(Debug)]
pub struct UpdatedDocument {
    pub changed: bool,
}