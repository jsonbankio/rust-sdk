/// About the current authenticated api key.
#[derive(Debug)]
pub struct AuthenticatedKey {
    /// The title of the api key.
    pub title: String,
    /// List of projects the api key has access to.
    pub projects: Vec<String>,
}

/// Holds the authentication data.
#[derive(Debug)]
pub struct AuthenticatedData {
    /// If the user is authenticated.
    pub authenticated: bool,
    /// The username of the authenticated user.
    pub username: String,
    /// About the current authenticated api key.
    pub api_key: AuthenticatedKey,
}

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


/// String and number information about the content size.
#[derive(Debug)]
pub struct ContentSize {
    pub number: u64,
    pub string: String,
}

/// Metadata about a document.
#[derive(Debug)]
pub struct DocumentMeta {
    /// The id of the document.
    pub id: String,
    /// The project the document belongs to.
    pub project: String,
    /// The path of the document.
    pub path: String,
    /// Name of the document.
    pub name: String,
    /// Optional folder the document belongs to.
    pub folder_id: Option<String>,
    /// The size of the document.
    pub content_size: ContentSize,
    /// The last time the document was updated.
    pub updated_at: String,
    /// The time the document was created.
    pub created_at: String,
}


/// The input body for creating a document.
#[derive(Debug)]
pub struct CreateDocumentBody {
    /// The name of the document.
    pub name: String,
    /// The project the document belongs to.
    pub project: String,
    /// The [String](String) content of the document.
    pub content: String,
    /// The folder the document belongs to. if not provided, the document will be created in the root of the project.
    pub folder: Option<String>,
}

/// The input body for creating a folder.
#[derive(Debug)]
pub struct CreateFolderBody {
    /// The name of the folder.
    pub name: String,
    /// The project the folder belongs to.
    pub project: String,
    /// The folder the document belongs to. if not provided, the folder will be created in the root of the project.
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

/// The input body for uploading a document.
#[derive(Debug)]
pub struct UploadDocumentBody {
    /// path of the file to upload
    pub file_path: String,
    /// The project the document belongs to.
    pub project: String,
    /// The name of the document. if not provided, the name of the file will be used.
    pub name: Option<String>,
    /// The folder the document belongs to. if not provided, the document will be created in the root of the project.
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

/// The input body for updating a document.
#[derive(Debug)]
pub struct NewDocument {
    /// The id of the document.
    pub id: String,
    /// The name of the document.
    pub name: String,
    /// The path of the document.
    pub path: String,
    /// The project the document belongs to.
    pub project: String,
    /// The time the document was created.
    pub created_at: String,

    /// If document was created or already exists
    /// this field is not returned by the api
    /// it is used by the `create_document_if_not_exists` function
    pub exists: bool,
}

/// Contains the number of documents and folders in a folder
#[derive(Debug)]
pub struct FolderStats {
    pub documents: i32,
    pub folders: i32,
}

/// About a folder.
#[derive(Debug)]
pub struct Folder {
    /// The id of the folder.
    pub id: String,
    /// The name of the folder.
    pub name: String,
    /// The path of the folder.
    pub path: String,
    /// The project the folder belongs to.
    pub project: String,
    /// The time the folder was created.
    pub created_at: String,
    /// The last time the folder was updated.
    pub updated_at: String,
    /// stats are only returned when the `include_stats` query parameter is set to true
    /// which is set to true in the `get_folder_stats` function
    pub stats: Option<FolderStats>,
}

/// Response from the api when a document is deleted.
#[derive(Debug)]
pub struct DeletedDocument {
    /// If `true`, the document was deleted else it was not deleted
    pub deleted: bool,
}

/// Response from the api when a document is updated.
#[derive(Debug)]
pub struct UpdatedDocument {
    /// If `true`, the document was updated else it was not updated
    /// if a document is not updated, it means the content is the same.
    pub changed: bool,
}

/// Pagination information returned alongside every paginated list.
#[derive(Debug)]
pub struct PaginationMeta {
    /// The page that was returned.
    pub page: i32,
    /// The number of results per page.
    pub per_page: i32,
    /// The total number of results across every page.
    pub total: i32,
    /// The last page available.
    pub last_page: i32,
}

/// A single page of documents.
#[derive(Debug)]
pub struct PaginatedDocuments {
    /// The documents on this page.
    pub data: Vec<DocumentMeta>,
    /// The pagination information for this list.
    pub meta: PaginationMeta,
}

/// A single page of folders.
#[derive(Debug)]
pub struct PaginatedFolders {
    /// The folders on this page.
    pub data: Vec<Folder>,
    /// The pagination information for this list.
    pub meta: PaginationMeta,
}

/// The project a listing belongs to.
#[derive(Debug)]
pub struct ListedProject {
    /// The slug of the project.
    pub slug: String,
    /// The title of the project.
    pub title: String,
    /// Either `public` or `private`.
    pub access: String,
}

/// The folder a listing was scoped to.
#[derive(Debug)]
pub struct ListedFolder {
    /// The id of the folder.
    pub id: String,
    /// The name of the folder.
    pub name: String,
    /// The path of the folder.
    pub path: String,
    /// The parent folder, when the listed folder is nested.
    pub parent_folder: Option<String>,
}

/// Response of the `scan_project` function.
#[derive(Debug)]
pub struct ScanProjectResponse {
    /// The project that was listed.
    pub project: ListedProject,
    /// The folder that was listed.
    /// `None` when the project root was listed.
    pub folder: Option<ListedFolder>,
    /// The page of documents.
    pub documents: PaginatedDocuments,
    /// The page of folders.
    pub folders: PaginatedFolders,
}

/// Response of the `list_documents` function.
#[derive(Debug)]
pub struct ListDocumentsResponse {
    /// The project that was listed.
    pub project: ListedProject,
    /// The folder that was listed.
    /// `None` when the project root was listed.
    pub folder: Option<ListedFolder>,
    /// The page of documents.
    pub documents: PaginatedDocuments,
}

/// Response of the `list_folders` function.
#[derive(Debug)]
pub struct ListFoldersResponse {
    /// The project that was listed.
    pub project: ListedProject,
    /// The folder that was listed.
    /// `None` when the project root was listed.
    pub folder: Option<ListedFolder>,
    /// The page of folders.
    pub folders: PaginatedFolders,
}

/// Query params of the `scan_project` function.
///
/// Fields left as `None` are not sent, so the server applies its own defaults.
/// Both lists paginate independently of each other, hence the separate pages.
///
/// Use `..Default::default()` to only set what you need:
/// ```
/// # use jsonbank::structs::ScanProjectParams;
/// let params = ScanProjectParams {
///     folder: Some("locales".to_string()),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default)]
pub struct ScanProjectParams {
    /// Folder to list, by id or by path. `None` lists the project root.
    pub folder: Option<String>,
    /// The page of documents to return.
    pub documents_page: Option<i32>,
    /// Documents per page, up to 1000.
    pub documents_per_page: Option<i32>,
    /// The page of folders to return.
    pub folders_page: Option<i32>,
    /// Folders per page, up to 1000.
    pub folders_per_page: Option<i32>,
    /// Field both lists are sorted by: `name`, `createdAt` or `updatedAt`.
    pub sort: Option<String>,
    /// Sort direction: `asc` or `desc`.
    pub order: Option<String>,
}

/// Query params of the `list_documents` and `list_folders` functions.
///
/// Fields left as `None` are not sent, so the server applies its own defaults.
/// Only one list is returned, so there is only one page to ask for.
///
/// Use `..Default::default()` to only set what you need:
/// ```
/// # use jsonbank::structs::ListParams;
/// let params = ListParams { per_page: Some(50), ..Default::default() };
/// ```
#[derive(Debug, Clone, Default)]
pub struct ListParams {
    /// Folder to list, by id or by path. `None` lists the project root.
    pub folder: Option<String>,
    /// The page to return.
    pub page: Option<i32>,
    /// Results per page, up to 1000.
    pub per_page: Option<i32>,
    /// Field to sort by: `name`, `createdAt` or `updatedAt`.
    pub sort: Option<String>,
    /// Sort direction: `asc` or `desc`.
    pub order: Option<String>,
}
