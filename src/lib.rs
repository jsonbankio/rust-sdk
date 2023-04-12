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
pub const DEFAULT_HOST: &str = "https://api.jsonbank.io";

// Keys struct - Public and private keys
struct Keys {
    public: Option<String>,
    private: Option<String>,
}

// Config struct - Host and keys and other config
pub struct Config {
    pub host: String,
    keys: Option<Keys>, // Keys
}

// Init Config - Minimal config needed to initialize
pub struct InitConfig {
    host: Option<String>,
    keys: Option<Keys>,
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

    // send_get_request - Sends get request
    // This function sends the http request using reqwest
    fn send_get_request(&self, url: String) -> Result<HashMap<String, Value>, JsbError> {
        // build request
        let client = reqwest::blocking::Client::new();
        // let res = client.get(&url).send()?;
        // use let res = match to handle error
        let res = match client.get(&url).send() {
            Ok(res) => res,
            Err(err) => {
                return Err(JsbError::from_any(&err, None));
            }
        };

        // Check if the response is successful
        if res.status().is_success() {
            let response_text = match res.text() {
                Ok(text) => text,
                Err(err) => {
                    return Err(JsbError::from_any(&err, None));
                }
            };

            // Deserialize the JSON response into a HashMap
            // let json: HashMap<String, Value> = serde_json::from_str(&response_text)?;

            // use match to handle error
            match serde_json::from_str(&response_text) {
                Ok(json) => Ok(json),
                Err(err) => Err(JsbError {
                    code: "500".to_string(),
                    message: err.to_string(),
                }),
            }
        } else {
            Err(JsbError {
                code: res.status().to_string(),
                message: "Request failed".to_string(),
            })
        }
    }

    // set_host - Sets host
    pub fn set_host(&mut self, host: &str) {
        self.config.host = host.to_string();
        // update endpoints
        self.endpoints = Self::make_endpoints(&self.config.host);
    }

    // get_document_meta - get public content meta from jsonbank
    pub fn get_document_meta(&self, id_or_path: &str) -> Result<DocumentMeta, JsbError> {
        let path = self.public_url(vec!["meta/f", id_or_path]);

        // use match to handle error
        match self.send_get_request(path) {
            Ok(res) => {
                // convert to DocumentMeta
                let meta = DocumentMeta {
                    id: res["id"].as_str().unwrap().to_string(),
                    project: res["project"].as_str().unwrap().to_string(),
                    path: res["path"].as_str().unwrap().to_string(),
                    updated_at: res["updatedAt"].as_str().unwrap().to_string(),
                    created_at: res["createdAt"].as_str().unwrap().to_string(),
                };

                Ok(meta)
            }
            Err(err) => Err(err),
        }
    }

    // get_content - get public content from jsonbank
    pub fn get_content(&self, id_or_path: &str) -> Result<HashMap<String, Value>, JsbError> {
        let path = self.public_url(vec!["f", id_or_path]);

        // send request and return response
        self.send_get_request(path)
    }

    // get_github_content - get content from github
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
