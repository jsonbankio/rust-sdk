// allow unused
#![allow(unused)]

extern crate reqwest;
extern crate serde;
extern crate serde_json;


mod jsb_error;
mod functions;
pub mod structs;

use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::{json, Value};
use std::any::Any;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::path::{Path, PathBuf};
use jsb_error::*;
use functions::*;
use structs::*;


pub const JSONBANK: &str = "jsonbank";
pub const JSONBANKIO: &str = "jsonbankio";
pub const DEFAULT_HOST: &str = "https://api.jsonbank.io";

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
    pub config: Config,
    // Config
    pub endpoints: Endpoints,
    // Endpoints
    // memory cache
    authenticated_data: Option<AuthenticatedData>,
}


// DocumentMeta struct - Document meta
#[derive(Debug)]
pub struct DocumentMeta {
    pub id: String,
    pub project: String,
    pub path: String,
    pub updated_at: String,
    pub created_at: String,
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

    // send_get_request - Sends get request
    // This function sends the http request using reqwest
    fn send_request<T: DeserializeOwned>(&self, method: &str, url: String, body: Option<HashMap<String, Value>>, require_pub_key: bool, require_prv_key: bool) -> Result<T, JsbError> {
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
        let res = match {
            match method {
                "POST" => client.post(&url).json(&body.unwrap_or(HashMap::new())),
                "DELETE" => client.delete(&url),
                _ => client.get(&url)
            }.headers(headers).send()
        } {
            Ok(res) => res,
            Err(err) => {
                return Err(JsbError::from_any(&err, None));
            }
        };

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
            let code = res.status().to_string();
            let data: HashMap<String, Value> = match res.json() {
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
    }

    // public_request - Sends get request to public endpoint
    fn public_request<T: DeserializeOwned>(&self, url: Vec<&str>) -> Result<T, JsbError> {
        self.send_request("GET", self.public_url(url), None, false, false)
    }

    // read_request - Sends get request to auth required endpoints using public key
    fn read_request<T: DeserializeOwned>(&self, url: Vec<&str>) -> Result<T, JsbError> {
        self.send_request("GET", self.v1_url(url), None, true, false)
    }

    // read_post_request - Sends post request to auth required endpoints using public key
    fn read_post_request<T: DeserializeOwned>(&self, url: Vec<&str>, body: Option<HashMap<String, Value>>) -> Result<T, JsbError> {
        self.send_request("POST", self.v1_url(url), body, true, false)
    }

    // write_request - Sends post request to auth required endpoints using private key
    fn write_request<T: DeserializeOwned>(&self, url: Vec<&str>, body: Option<HashMap<String, Value>>) -> Result<T, JsbError> {
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
        match self.public_request::<HashMap<String, Value>>(vec!["meta/f", id_or_path]) {
            Ok(res) => {
                // convert to DocumentMeta
                Ok(hash_map_to_document_meta(&res))
            }
            Err(err) => Err(err),
        }
    }

    // get_content - get public content from jsonbank
    pub fn get_content<T: DeserializeOwned>(&self, id_or_path: &str) -> Result<T, JsbError> {
        self.public_request::<T>(vec!["f", id_or_path])
    }

    // get_github_content - get content from github
    pub fn get_github_content<T: DeserializeOwned>(&self, path: &str) -> Result<T, JsbError> {
        self.public_request(vec!["gh", path])
    }
}


// Auth Implementation
impl JsonBank {
    // authenticate - Authenticate user
    pub fn authenticate(&mut self) -> Result<AuthenticatedData, JsbError> {
        match self.read_post_request::<HashMap<String, Value>>(vec!["authenticate"], None) {
            Ok(res) => {
                // convert to AuthenticatedData
                let mut data = AuthenticatedData {
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
        match self.read_request::<HashMap<String, Value>>(vec!["meta/file", path]) {
            Ok(res) => {
                // convert to DocumentMeta
                Ok(hash_map_to_document_meta(&res))
            }
            Err(err) => Err(err),
        }
    }


    // get_own_content - get own content from jsonbank
    pub fn get_own_content<T: DeserializeOwned>(&self, path: &str) -> Result<T, JsbError> {
        self.read_request(vec!["file", path])
    }

    // has_own_document - check if user has document
    // This method will try to get document meta and if it fails it will return false
    pub fn has_own_document(&self, path: &str) -> bool {
        self.get_own_document_meta(path).is_ok()
    }

    // create_document - create a document
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
            return Err(JsbError {
                code: "bad_request".to_string(),
                message: "Content is not valid json".to_string(),
            });
        }

        // convert content to hashmap
        let mut body: HashMap<String, Value> = HashMap::from([
            ("name".to_string(), Value::String(content.name)),
            ("project".to_string(), Value::String(content.project.clone())),
            ("content".to_string(), Value::String(content.content)),
        ]);

        // add folder if set
        if content.folder.is_some() {
            body.insert("folder".to_string(), Value::String(content.folder.unwrap()));
        }


        // send request
        let url = vec!["project", &content.project, "document"];
        match self.write_request::<HashMap<String, Value>>(url, Some(body)) {
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

        // let path = Path::new(&doc.file_path).
        let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(doc.file_path);
        println!("file_path: {:?}", file_path);
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
            return Err(JsbError {
                code: "invalid_file".to_string(),
                message: "File is not valid json".to_string(),
            });
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
        match self.delete_request::<HashMap<String, Value>>(vec!["file", path]) {
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
            },
        }
    }
}