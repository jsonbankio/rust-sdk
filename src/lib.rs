// allow unused
#![allow(unused)]

extern crate reqwest;

use reqwest::Error;
use std::any::Any;
use std::collections::HashMap;
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
    pub fn send_get_request(&self, url: String) -> Result<(), Error> {
        // build request
        let client = reqwest::blocking::Client::new();
        let res = client.get(url).send()?;

        // print response
        println!("Response: {}", res.text()?);

        // return Ok
        Ok(())
    }

    // GetContent - get public content from jsonbank
    pub fn get_content(&self, id_or_path: &str) -> Result<(), Error> {
        let path = self.public_url(vec!["f", id_or_path]);

        // send request
        self.send_get_request(path)?;

        // return Ok
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Data {
        pub project: &'static str,
        pub file: &'static str,
    }

    fn init() -> (JsonBank, Data) {
        let mut jsb = JsonBank::new_without_config();
        // set host to dev server
        jsb.set_host("http://localhost:2223");

        // let mut data = HashMap::new();

        // data.insert("project", "jsonbank/sdk-test");
        // data.insert("file", "index.json");

        let data = Data {
            project: "jsonbank/sdk-test",
            file: "index.json",
        };

        (jsb, data)
    }

    #[test]
    fn get_content() {
        let (jsb, data) = init();

        // get project
        let project = data.project;
        let file = data.file;
        let path = format!("{}/{}", project, file);

        // get content
        jsb.get_content(&path).unwrap();
    }
}
