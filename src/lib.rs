// allow unused
// #![allow(unused)]

extern crate reqwest;
extern crate serde;
extern crate serde_json;


mod functions;
pub mod jsb_error;
pub mod structs;

use serde::{de::DeserializeOwned};
use serde_json::{Value};
use std::collections::HashMap;
use std::fs;
use std::path::{PathBuf};
use reqwest::blocking::Response;
use jsb_error::*;
use functions::*;
use structs::*;


pub const JSONBANK: &str = "jsonbank";
pub const JSONBANK_IO: &str = "jsonbankio";
pub const DEFAULT_HOST: &str = "https://api.jsonbank.io";

// JsonValue type - Json value is an alias for serde_json::Value
// so adding serde_json as a dependency is not necessary
pub type JsonValue = Value;

// JsonObject type - Json object is an alias for HashMap<String, JsonValue>
pub type JsonObject = HashMap<String, JsonValue>;

// JsonArray type - Json Array is an alias for Vec<JsonValue>
pub type JsonArray = Vec<JsonValue>;

// Keys struct - Public and private keys
pub struct Keys {
    pub public: Option<String>,
    pub private: Option<String>,
}

// Config struct - Host and keys and other config
pub struct Config {
    pub host: String,
    keys: Option<Keys>, // Keys
}

// Init Config - Minimal config needed to initialize
pub struct InitConfig {
    pub host: Option<String>,
    pub keys: Option<Keys>,
}

// Endpoints struct - Endpoints
pub struct Endpoints {
    pub v1: String,
    pub public: String,
}

// JsonBank struct - Sdk Instance
pub struct JsonBank {
    // Config
    pub config: Config,
    // Endpoints
    pub endpoints: Endpoints,
    // Authenticated data
    authenticated_data: Option<AuthenticatedData>,
}



// Implementing JsonBank
impl JsonBank {
    // has_key - Checks if a key is provided either public or private
    fn has_key(&self, key: &str) -> bool {
        // check if keys are provided
        if self.config.keys.is_none() {
            return false;
        }

        // get keys
        let keys = self.config.keys.as_ref().unwrap();

        // check if key is provided
        if key == "public" {
            return keys.public.is_some();
        } else if key == "private" {
            return keys.private.is_some();
        }

        false
    }

    // get_key - Returns the key
    fn get_key(&self, key: &str) -> String {
        // check if keys are provided
        if self.config.keys.is_none() {
            return "".to_string();
        }

        // get keys
        let keys = self.config.keys.as_ref().unwrap();

        // check if key is provided
        if key == "public" {
            return keys.public.as_ref().unwrap().to_string();
        } else if key == "private" {
            return keys.private.as_ref().unwrap().to_string();
        }

        "".to_string()
    }

    // Make Endpoints
    fn make_endpoints(host: &String) -> Endpoints {
        Endpoints {
            v1: format!("{}/v1", host),
            public: host.to_string(),
        }
    }

    // New method - Returns JsonBank struct
    pub fn new(conf: InitConfig) -> Self {
        let host = conf.host.unwrap_or(DEFAULT_HOST.to_string());

        // build config
        let config = Config {
            host: host.to_string(),
            keys: conf.keys,
        };

        // set endpoints
        let endpoints = Self::make_endpoints(&host);

        // return JsonBank struct
        JsonBank { config, endpoints, authenticated_data: None }
    }

    // New using default config - Returns JsonBank struct
    pub fn new_without_config() -> Self {
        Self::new(InitConfig {
            host: None,
            keys: None,
        })
    }
}

// Instance Implementation
impl JsonBank {
    // Format public url
    fn public_url(&self, paths: Vec<&str>) -> String {
        // add paths to public endpoint
        format!("{}/{}", self.endpoints.public, paths.join("/"))
    }

    // format v1 url
    fn v1_url(&self, paths: Vec<&str>) -> String {
        // add paths to v1 endpoint
        format!("{}/{}", self.endpoints.v1, paths.join("/"))
    }

    // process_response_error - Processes response error
    fn process_response_error<T: DeserializeOwned>(&self, res: Response) -> Result<T, JsbError> {
        let code = res.status().to_string();
        let data: JsonObject = match res.json() {
            Ok(text) => text,
            Err(err) => {
                return Err(JsbError::from_any(&err, None));
            }
        };

        // get error object from data
        let error = match data["error"].as_object() {
            Some(err) => err,
            None => {
                return Err(JsbError {
                    code,
                    message: "Unknown error".to_string(),
                });
            }
        };


        Err(JsbError {
            code: error["code"].as_str().unwrap().to_string(),
            message: error["message"].as_str().unwrap().to_string(),
        })
    }

    // process_response - Processes response
    fn process_response<T: DeserializeOwned>(&self, res: Response) -> Result<T, JsbError> {
        // Check if the response is successful
        if res.status().is_success() {
            let response_text: T = match res.json() {
                Ok(text) => text,
                Err(err) => {
                    return Err(JsbError::from_any(&err, None));
                }
            };

            Ok(response_text)
        } else {
            self.process_response_error(res)
        }
    }

    // process_response_as_string - Processes response as text
    fn process_response_as_string(&self, res: Response) -> Result<String, JsbError> {
        // Check if the response is successful
        if res.status().is_success() {
            let response_text: String = match res.text() {
                Ok(text) => text,
                Err(err) => {
                    return Err(JsbError::from_any(&err, None));
                }
            };

            Ok(response_text)
        } else {
            self.process_response_error(res)
        }
    }

    // make_request - Makes request
    fn make_request(&self, method: &str, url: String, body: Option<JsonObject>, require_pub_key: bool, require_prv_key: bool) -> Result<Response, JsbError> {
        // build request
        let client = reqwest::blocking::Client::new();
        // add json header
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());

        // check if public key is required and not provided
        if require_pub_key {
            if self.has_key("public") {
                // add public key to headers as `jsb-pub-key`
                headers.insert("jsb-pub-key", self.get_key("public").parse().unwrap());
            } else {
                return Err(JsbError {
                    code: "bad_request".to_string(),
                    message: "Public key is not set".to_string(),
                });
            }
        }

        // check if private key is required and not provided
        if require_prv_key {
            if self.has_key("private") {
                // add private key to headers as `jsb-private-key`
                headers.insert("jsb-prv-key", self.get_key("private").parse().unwrap());
            } else {
                return Err(JsbError {
                    code: "bad_request".to_string(),
                    message: "Private key is not set".to_string(),
                });
            }
        }

        // send request
        match {
            match method {
                "POST" => client.post(&url).json(&body.unwrap_or(HashMap::new())),
                "DELETE" => client.delete(&url),
                _ => client.get(&url).query(&body.unwrap_or(HashMap::new())),
            }.headers(headers).send()
        } {
            Ok(res) => Ok(res),
            Err(err) => {
                return Err(JsbError::from_any(&err, None));
            }
        }
    }

    // send_get_request - Sends get request
    // This function sends the http request using reqwest
    fn send_request<T: DeserializeOwned>(&self, method: &str, url: String, body: Option<JsonObject>, require_pub_key: bool, require_prv_key: bool) -> Result<T, JsbError> {
        // make request
        let res = match self.make_request(method, url, body, require_pub_key, require_prv_key) {
            Ok(res) => res,
            Err(err) => {
                return Err(err);
            }
        };

        // process response
        self.process_response(res)
    }

    // send_request_as_string - Sends request and returns response as text
    fn send_request_as_string(&self, method: &str, url: String, body: Option<JsonObject>, require_pub_key: bool, require_prv_key: bool) -> Result<String, JsbError> {
        // make request
        let res = match self.make_request(method, url, body, require_pub_key, require_prv_key) {
            Ok(res) => res,
            Err(err) => {
                return Err(err);
            }
        };

        // process response
        self.process_response_as_string(res)
    }

    // public_request - Sends get request to public endpoint
    fn public_request<T: DeserializeOwned>(&self, url: Vec<&str>) -> Result<T, JsbError> {
        self.send_request("GET", self.public_url(url), None, false, false)
    }

    // public_request_as_string - Sends get request to public endpoint and returns response as text
    fn public_request_as_string(&self, url: Vec<&str>) -> Result<String, JsbError> {
        self.send_request_as_string("GET", self.public_url(url), None, false, false)
    }

    // read_request - Sends get request to auth required endpoints using public key
    fn read_request<T: DeserializeOwned>(&self, url: Vec<&str>, query: Option<JsonObject>) -> Result<T, JsbError> {
        self.send_request("GET", self.v1_url(url), query, true, false)
    }

    // read_request_as_string - Sends get request to auth required endpoints using public key and returns response as text
    fn read_request_as_string(&self, url: Vec<&str>, query: Option<JsonObject>) -> Result<String, JsbError> {
        self.send_request_as_string("GET", self.v1_url(url), query, true, false)
    }

    // read_post_request - Sends post request to auth required endpoints using public key
    fn read_post_request<T: DeserializeOwned>(&self, url: Vec<&str>, body: Option<JsonObject>) -> Result<T, JsbError> {
        self.send_request("POST", self.v1_url(url), body, true, false)
    }

    // write_request - Sends post request to auth required endpoints using private key
    fn write_request<T: DeserializeOwned>(&self, url: Vec<&str>, body: Option<JsonObject>) -> Result<T, JsbError> {
        self.send_request("POST", self.v1_url(url), body, false, true)
    }

    // delete_request - Sends delete request to auth required endpoints using private key
    fn delete_request<T: DeserializeOwned>(&self, url: Vec<&str>) -> Result<T, JsbError> {
        self.send_request("DELETE", self.v1_url(url), None, false, true)
    }

    // set_host - Sets host
    pub fn set_host(&mut self, host: &str) {
        self.config.host = host.to_string();
        // update endpoints
        self.endpoints = Self::make_endpoints(&self.config.host);
    }

    // get_document_meta - get public content meta from jsonbank
    pub fn get_document_meta(&self, id_or_path: &str) -> Result<DocumentMeta, JsbError> {
        match self.public_request::<JsonObject>(vec!["meta/f", id_or_path]) {
            Ok(res) => {
                // convert to DocumentMeta
                Ok(json_object_to_document_meta(&res))
            }
            Err(err) => Err(err),
        }
    }

    // get_content - get public content from jsonbank
    pub fn get_content<T: DeserializeOwned>(&self, id_or_path: &str) -> Result<T, JsbError> {
        self.public_request::<T>(vec!["f", id_or_path])
    }

    // get_content_as_string - get public content as string from jsonbank
    pub fn get_content_as_string(&self, id_or_path: &str) -> Result<String, JsbError> {
        self.public_request_as_string(vec!["f", id_or_path])
    }

    // get_github_content - get content from github
    pub fn get_github_content<T: DeserializeOwned>(&self, path: &str) -> Result<T, JsbError> {
        self.public_request(vec!["gh", path])
    }

    // get_github_content_as_string - get content as string from github
    pub fn get_github_content_as_string(&self, path: &str) -> Result<String, JsbError> {
        self.public_request_as_string(vec!["gh", path])
    }
}


// Auth Implementation
impl JsonBank {
    // authenticate - Authenticate user
    pub fn authenticate(&mut self) -> Result<AuthenticatedData, JsbError> {
        match self.read_post_request::<JsonObject>(vec!["authenticate"], None) {
            Ok(res) => {
                // convert to AuthenticatedData
                let data = AuthenticatedData {
                    authenticated: res["authenticated"].as_bool().unwrap(),
                    username: res["username"].as_str().unwrap().to_string(),
                    api_key: AuthenticatedKey {
                        title: res["apiKey"]["title"].as_str().unwrap().to_string(),
                        projects: res["apiKey"]["projects"].as_array().unwrap().iter().map(|x| x.as_str().unwrap().to_string()).collect(),
                    },
                };

                // set authenticated data
                self.authenticated_data = Some(data.clone());

                Ok(data)
            }
            Err(err) => Err(err),
        }
    }

    // get_authenticated_data - Get authenticated data
    pub fn get_username(&self) -> Result<String, JsbError> {
        match &self.authenticated_data {
            Some(data) => Ok(data.username.clone()),
            None => Err(JsbError {
                code: "not_authenticated".to_string(),
                message: "User is not authenticated".to_string(),
            }),
        }
    }

    // is_authenticated - Check if user is authenticated
    pub fn is_authenticated(&self) -> bool {
        match &self.authenticated_data {
            Some(data) => data.authenticated,
            None => false,
        }
    }

    // get_own_document_meta - get own content meta from jsonbank
    pub fn get_own_document_meta(&self, path: &str) -> Result<DocumentMeta, JsbError> {
        match self.read_request::<JsonObject>(vec!["meta/file", path], None) {
            Ok(res) => {
                // convert to DocumentMeta
                Ok(json_object_to_document_meta(&res))
            }
            Err(err) => Err(err),
        }
    }


    // get_own_content - get own content from jsonbank
    pub fn get_own_content<T: DeserializeOwned>(&self, path: &str) -> Result<T, JsbError> {
        self.read_request(vec!["file", path], None)
    }

    // get_own_content_as_string - get own content as string from jsonbank
    pub fn get_own_content_as_string(&self, path: &str) -> Result<String, JsbError> {
        self.read_request_as_string(vec!["file", path], None)
    }

    // has_own_document - check if user has document
    // This method will try to get document meta and if it fails it will return false
    pub fn has_own_document(&self, path: &str) -> bool {
        self.get_own_document_meta(path).is_ok()
    }

    // create_document - create a document.
    pub fn create_document(&self, content: CreateDocumentBody) -> Result<NewDocument, JsbError> {

        // check if content.project is set
        if content.project.is_empty() {
            return Err(JsbError {
                code: "bad_request".to_string(),
                message: "Project required".to_string(),
            });
        }

        // check if content.name is set
        if content.name.is_empty() {
            return Err(JsbError {
                code: "bad_request".to_string(),
                message: "Name required".to_string(),
            });
        }

        // check if content.content is set
        if content.content.is_empty() {
            return Err(JsbError {
                code: "bad_request".to_string(),
                message: "Content required".to_string(),
            });
        }

        // check if content.content is a valid json
        if !is_valid_json(&content.content) {
            return Err(err_invalid_json());
        }

        // convert content to hashmap
        let mut body: JsonObject = HashMap::from([
            ("name".to_string(), JsonValue::String(content.name)),
            ("project".to_string(), JsonValue::String(content.project.clone())),
            ("content".to_string(), JsonValue::String(content.content)),
        ]);

        // add folder if set
        if content.folder.is_some() {
            body.insert("folder".to_string(), JsonValue::String(content.folder.unwrap()));
        }


        // send request
        let url = vec!["project", &content.project, "document"];
        match self.write_request::<JsonObject>(url, Some(body)) {
            Ok(res) => {
                // convert to NewDocument
                Ok(NewDocument {
                    id: res["id"].as_str().unwrap().to_string(),
                    name: res["name"].as_str().unwrap().to_string(),
                    path: res["path"].as_str().unwrap().to_string(),
                    project: res["project"].as_str().unwrap().to_string(),
                    created_at: res["createdAt"].as_str().unwrap().to_string(),
                    exists: false,
                })
            }
            Err(err) => Err(err),
        }
    }

    // create_document_if_not_exists - create a document if it does not exist
    // First, it will try to create the document, if it fails and document error code is "name.exists" it will try to get the document
    // and return it
    pub fn create_document_if_not_exists(&self, content: CreateDocumentBody) -> Result<NewDocument, JsbError> {
        match self.create_document(content.clone()) {
            Ok(res) => Ok(res),
            Err(err) => {
                // check if error code is name.exists
                if err.code == "name.exists" {
                    let doc_path = make_document_path(&content);
                    // get document
                    match self.get_own_document_meta(doc_path.as_str()) {
                        Ok(res) => Ok(NewDocument {
                            id: res.id,
                            name: content.name,
                            path: res.path,
                            project: res.project,
                            created_at: res.created_at,
                            exists: true,
                        }),
                        Err(err) => Err(err),
                    }
                } else {
                    Err(err)
                }
            }
        }
    }


    // update_own_document - update a document
    pub fn update_own_document(&self, id_or_path: &str, content: String) -> Result<UpdatedDocument, JsbError> {
        // check if content is a valid json
        if !is_valid_json(&content) {
            return Err(err_invalid_json());
        }

        // create body
        let body = JsonObject::from([
            ("content".to_string(), JsonValue::String(content)),
        ]);

        // send request
        let url = vec!["file", id_or_path];

        match self.write_request::<JsonObject>(url, Some(body)) {
            Ok(res) => {
                // convert to UpdatedDocument
                Ok(UpdatedDocument {
                    changed: res["changed"].as_bool().unwrap_or(false),
                })
            }
            Err(err) => Err(err),
        }
    }


    // upload document - upload a json document
    // this method will read the file and upload it to jsonbank
    pub fn upload_document(&self, doc: UploadDocumentBody) -> Result<NewDocument, JsbError> {
        // project is required
        if doc.project.is_empty() {
            return Err(JsbError {
                code: "bad_request".to_string(),
                message: "Project required".to_string(),
            });
        }

        let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(doc.file_path);

        // check if file exists using os
        if !file_path.exists() {
            return Err(JsbError {
                code: "file_not_found".to_string(),
                message: "File does not exist".to_string(),
            });
        }

        // read file
        let file_content = match fs::read_to_string(file_path.clone()) {
            Ok(res) => res,
            Err(err) => {
                return Err(JsbError {
                    code: "invalid_file".to_string(),
                    message: err.to_string(),
                });
            }
        };

        // check if file is valid json
        if !is_valid_json(&file_content) {
            return Err(err_invalid_json());
        }

        // set name if not set
        let name = if doc.name.is_none() {
            file_path.file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        } else {
            doc.name.unwrap()
        };

        self.create_document(CreateDocumentBody {
            name,
            project: doc.project,
            content: file_content,
            folder: doc.folder,
        })
    }

    // delete_document - delete a document
    pub fn delete_document(&self, path: &str) -> Result<DeletedDocument, JsbError> {
        match self.delete_request::<JsonObject>(vec!["file", path]) {
            Ok(res) => {
                // convert to DeletedDocument
                Ok(DeletedDocument {
                    deleted: res["deleted"].as_bool().unwrap_or(false),
                })
            }
            Err(err) => {
                // if error code is `notFound` return DeletedDocument with deleted = false
                if err.code == "notFound" {
                    Ok(DeletedDocument { deleted: false })
                } else {
                    Err(err)
                }
            }
        }
    }

    // create_folder - create a folder
    pub fn create_folder(&self, data: CreateFolderBody) -> Result<Folder, JsbError> {
        // project is required
        if data.project.is_empty() {
            return Err(JsbError {
                code: "bad_request".to_string(),
                message: "Project required".to_string(),
            });
        }

        // name is required
        if data.name.is_empty() {
            return Err(JsbError {
                code: "bad_request".to_string(),
                message: "Name required".to_string(),
            });
        }

        // create body
        let body = JsonObject::from([
            ("name".to_string(), JsonValue::String(data.name)),
            ("project".to_string(), JsonValue::String(data.project.clone())),
        ]);

        // send request
        let url = vec!["project", &data.project, "folder"];
        match self.write_request::<JsonObject>(url, Some(body)) {
            Ok(res) => {
                // convert to NewFolder
                Ok(json_object_to_folder(&res))
            }
            Err(err) => Err(err),
        }
    }

    //  private _get_folder - get a folder
    fn ___get_folder(&self, path: &str, include_stats: bool) -> Result<Folder, JsbError> {
        // create query
        let query = if include_stats {
            Some(JsonObject::from([
                ("stats".to_string(), JsonValue::Bool(true)),
            ]))
        } else {
            None
        };

        match self.read_request::<JsonObject>(vec!["folder", path], query) {
            Ok(res) => {
                // convert to Folder
                Ok(json_object_to_folder(&res))
            }
            Err(err) => Err(err),
        }
    }

    // get_folder - get a folder
    pub fn get_folder(&self, path: &str) -> Result<Folder, JsbError> {
        self.___get_folder(path, false)
    }

    // get_folder_with_stats - get a folder with stats
    pub fn get_folder_with_stats(&self, path: &str) -> Result<Folder, JsbError> {
        self.___get_folder(path, true)
    }

    // create_folder_if_not_exists - create a folder if it does not exist
    // First, it will try to create the folder, if it fails and folder error code is "name.exists" it will try to get the folder
    // and return it.
    pub fn create_folder_if_not_exists(&self, data: CreateFolderBody) -> Result<(Folder, bool), JsbError> {
        match self.create_folder(data.clone()) {
            Ok(res) => Ok((res, false)),
            Err(err) => {
                // check if error code is name.exists
                if err.code == "name.exists" {
                    let folder_path = make_folder_path(&data);
                    // get folder
                    match self.get_folder(folder_path.as_str()) {
                        Ok(res) => Ok((res, true)),
                        Err(err) => Err(err),
                    }
                } else {
                    Err(err)
                }
            }
        }
    }
}