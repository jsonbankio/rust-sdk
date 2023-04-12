// allow unused
#![allow(unused)]

extern crate reqwest;
extern crate serde;
extern crate serde_json;

use reqwest::blocking::get;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::Value;
use std::any::Any;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

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

    // Set host - Sets host
    pub fn set_host(&mut self, host: &str) {
        self.config.host = host.to_string();
        // update endpoints
        self.endpoints = Self::make_endpoints(&self.config.host);
    }

    // Send request
    // This function sends the http request using reqwest
    pub fn send_get_request(&self, url: String) -> Result<HashMap<String, Value>, Box<dyn Error>> {
        // build request
        let client = reqwest::blocking::Client::new();
        let res = client.get(url).send()?;

        // Check if the response is successful
        if res.status().is_success() {
            let response_text = res.text()?;

            // Deserialize the JSON response into a HashMap
            let json: HashMap<String, Value> = serde_json::from_str(&response_text)?;

            Ok(json)
        } else {
            Err("GET request failed with non-successful status code".into())
        }
    }

    // GetDocumentMeta - get public content meta from jsonbank
    pub fn get_document_meta(&self, id_or_path: &str) -> Result<DocumentMeta, Box<dyn Error>> {
        let path = self.public_url(vec!["meta/f", id_or_path]);

        // send request
        let res = self.send_get_request(path)?;

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

    // GetContent - get public content from jsonbank
    pub fn get_content(&self, id_or_path: &str) -> Result<HashMap<String, Value>, Box<dyn Error>> {
        let path = self.public_url(vec!["f", id_or_path]);

        // send request and return response
        self.send_get_request(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct Data {
        pub project: &'static str,
        pub file: &'static str,
        pub id: Option<String>,
    }

    fn init() -> (JsonBank, Data) {
        let mut jsb = JsonBank::new_without_config();
        // set host to dev server
        jsb.set_host("http://localhost:2223");

        let mut data = Data {
            project: "jsonbank/sdk-test",
            file: "index.json",
            id: None,
        };

        // get metadata for test file
        let path = format!("{}/{}", data.project, data.file);
        let meta = jsb.get_document_meta(&path).unwrap();

        // update id
        data.id = Some(meta.id);

        println!("meta: {:?}", data);

        (jsb, data)
    }

    #[test]
    fn get_content() {
        let (jsb, data) = init();

        // set path
        // let path = format!("{}/{}", data.project, data.file);

        // get content
        // jsb.get_content(&path).unwrap();
    }
}
