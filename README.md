# JsonBank.io Rust SDK

The official repository for the JsonBank.io Rust SDK.


## Usage

Add the following to your Cargo.toml file

```toml
[dependencies]
jsonbank = "0.1.0"
```

Then import the library in your code

```rust
use jsonbank::JsonBank;
use std::collections::HashMap;
use serde_json::{Value};

fn main() {
    let jsb = JsonBank::new_without_config();

    let content: HashMap<String, Value> = match jsb.get_content("jsonbank/sdk-test/index.json") {
        Ok(content) => content,
        Err(err) => panic!("{:?}", err),
    };
    
    println!("{:?}", content);
}
```


## Testing
Create an .env file in the root of the project and add the following variables

```dotenv
JSB_HOST="https://api.jsonbank.io"
JSB_PUBLIC_KEY="your public key"
JSB_PRIVATE_KEY="your private key"
```

Then run the tests command below.

Note: A single thread is required for test so that all tests can run in defined order.
```bash
cargo test -- --test-threads=1
```