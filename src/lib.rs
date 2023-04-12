// allow unused
#![allow(unused)]

extern crate reqwest;

use std::collections::HashMap;
use reqwest::Error;

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


    // Set host - Sets host
    pub fn set_host(&mut self, host: &str) {
        self.config.host = host.to_string();
        // update endpoints
        self.endpoints = Self::make_endpoints(&self.config.host);
    }

    // Send request
    // This function sends the http request using reqwest
    pub fn send_request(&self) -> Result<(), Error> {
        // build request
        let client = reqwest::blocking::Client::new();
        let res = client.get(&self.endpoints.public).send()?;

        // print response
        println!("Response: {}", res.text()?);

        // return Ok
        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
     fn can_send_request() {
        let mut jsb = JsonBank::new_without_config();
        // set host to dev server
        jsb.set_host("http://localhost:2223");

        jsb.send_request();
    }
}
