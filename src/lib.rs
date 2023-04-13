// allow unused
#![allow(unused)]

extern crate reqwest;
extern crate serde;
extern crate serde_json;

use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::Value;
use std::any::Any;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};


pub const JSONBANK: &str = "jsonbank";
pub const JSONBANKIO: &str = "jsonbankio";
pub const DEFAULT_HOST: &str = "https://api.jsonbank.io";

// Keys struct - Public and private keys
// TODO: remove debug
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
    pub endpoints: Endpoints, // Endpoints
}

// JsbError struct - Error struct
#[derive(Debug)]
pub struct JsbError {
    pub code: String,
    pub message: String,
}

impl Display for JsbError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for JsbError {}

impl JsbError {
    // convert any error to jsb error
    pub fn from_any(err: &dyn Any, _code: Option<&str>) -> JsbError {
        let mut code = _code.unwrap_or("500");

        if let Some(err) = err.downcast_ref::<JsbError>() {
            // if _code is not provided, use the code from the error
            if _code.is_none() {
                code = &err.code;
            }

            JsbError {
                code: code.to_string(),
                message: err.message.to_string(),
            }
        } else if let Some(err) = err.downcast_ref::<reqwest::Error>() {
            JsbError {
                code: code.to_string(),
                message: err.to_string(),
            }
        } else if let Some(err) = err.downcast_ref::<serde_json::Error>() {
            JsbError {
                code: code.to_string(),
                message: err.to_string(),
            }
        } else {
            JsbError {
                code: code.to_string(),
                message: "Unknown error".to_string(),
            }
        }
    }
}

// API Error struct
#[derive(Debug)]
pub struct ApiError {
    pub code: String,
    pub message: String,
}

// Api Error Response struct
#[derive(Debug)]
pub struct ApiErrorResponse {
    pub error: ApiError,
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

fn hash_map_to_document_meta(map: &HashMap<String, Value>) -> DocumentMeta {
    DocumentMeta {
        id: map["id"].as_str().unwrap().to_string(),
        project: map["project"].as_str().unwrap().to_string(),
        path: map["path"].as_str().unwrap().to_string(),
        updated_at: map["updatedAt"].as_str().unwrap().to_string(),
        created_at: map["createdAt"].as_str().unwrap().to_string(),
    }
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
        JsonBank { config, endpoints }
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
    fn send_get_request<T: DeserializeOwned>(&self, url: String, require_pub_key: bool, require_prv_key: bool) -> Result<T, JsbError> {
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


        let res = match client.get(&url).headers(headers).send() {
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

    // public_get_request - Sends get request to public endpoint
    fn public_get_request<T: DeserializeOwned>(&self, url: Vec<&str>) -> Result<T, JsbError> {
        self.send_get_request(self.public_url(url), false, false)
    }

    // auth_get_request - Sends get request to auth required endpoints using public key
    fn auth_get_request<T: DeserializeOwned>(&self, url: Vec<&str>) -> Result<T, JsbError> {
        self.send_get_request(self.v1_url(url), true, false)
    }

    // set_host - Sets host
    pub fn set_host(&mut self, host: &str) {
        self.config.host = host.to_string();
        // update endpoints
        self.endpoints = Self::make_endpoints(&self.config.host);
    }

    // get_document_meta - get public content meta from jsonbank
    pub fn get_document_meta(&self, id_or_path: &str) -> Result<DocumentMeta, JsbError> {
        match self.public_get_request::<HashMap<String, Value>>(vec!["meta/f", id_or_path]) {
            Ok(res) => {
                // convert to DocumentMeta
                Ok(hash_map_to_document_meta(&res))
            }
            Err(err) => Err(err),
        }
    }

    // get_content - get public content from jsonbank
    pub fn get_content<T: DeserializeOwned>(&self, id_or_path: &str) -> Result<T, JsbError> {
        self.public_get_request::<T>(vec!["f", id_or_path])
    }

    // get_github_content - get content from github
    pub fn get_github_content<T: DeserializeOwned>(&self, path: &str) -> Result<T, JsbError> {
        self.public_get_request(vec!["gh", path])
    }
}


// Auth Implementation
impl JsonBank {
    // get_own_document_meta - get own content meta from jsonbank
    pub fn get_own_document_meta(&self, path: &str) -> Result<DocumentMeta, JsbError> {
        match self.auth_get_request::<HashMap<String, Value>>(vec!["meta/file", path]) {
            Ok(res) => {
                // convert to DocumentMeta
                Ok(hash_map_to_document_meta(&res))
            }
            Err(err) => Err(err),
        }
    }


    // get_own_content - get own content from jsonbank
    pub fn get_own_content<T: DeserializeOwned>(&self, path: &str) -> Result<T, JsbError> {
        self.auth_get_request(vec!["file", path])
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//
//     #[derive(Debug)]
//     struct TestData {
//         pub project: &'static str,
//         pub file: &'static str,
//         pub id: Option<String>,
//         pub path: String,
//     }
//
//     // user_path - returns path for user
//     fn user_path(path: String) -> String {
//         format!("{}/{}", JSONBANK, path)
//     }
//
//
//     // init - initialize test
//     fn init() -> (JsonBank, TestData) {
//         let mut jsb = JsonBank::new_without_config();
//         // set host to dev server
//         jsb.set_host("http://localhost:2223");
//
//         let mut data = TestData {
//             project: "jsonbank/sdk-test",
//             file: "index.json",
//             id: None,
//             path: "".to_string(),
//         };
//
//         // set path
//         data.path = format!("{}/{}", data.project, data.file);
//
//         // get metadata for test file
//         let path = format!("{}/{}", data.project, data.file);
//         let meta = jsb.get_document_meta(&path).unwrap();
//
//         // update id
//         data.id = Some(meta.id);
//
//         (jsb, data)
//     }
//
//     #[test]
//     fn get_content() {
//         let (jsb, mut data) = init();
//
//         // get content by id
//         let content = match jsb.get_content(&data.id.unwrap()) {
//             Ok(content) => content,
//             Err(err) => panic!("{:?}", err),
//         };
//
//         assert_eq!(content["author"], JSONBANK);
//
//         // get content by path
//         let content = match jsb.get_content(&data.path) {
//             Ok(content) => content,
//             Err(err) => panic!("{:?}", err),
//         };
//
//         assert_eq!(content["author"], JSONBANK);
//     }
//
//
//     #[test]
//     fn get_document_meta() {
//         let (jsb, data) = init();
//
//         // get metadata by id
//         let meta = match jsb.get_document_meta(&data.id.unwrap()) {
//             Ok(meta) => meta,
//             Err(err) => panic!("{:?}", err),
//         };
//
//         assert_eq!(user_path(meta.project), data.project);
//         assert_eq!(meta.path, data.file);
//
//         // get metadata by path
//         let meta = match jsb.get_document_meta(&data.path) {
//             Ok(meta) => meta,
//             Err(err) => panic!("{:?}", err),
//         };
//
//         assert_eq!(user_path(meta.project), data.project);
//         assert_eq!(meta.path, data.file);
//     }
// }
