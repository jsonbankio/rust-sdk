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
use jsonbank::{JsonBank, JsonObject};

fn main() {
    let mut jsb = JsonBank::new_without_config();
    // set host to dev server
    jsb.set_host("http://localhost:2223");

    // Call the send_get_request() function
    let data: JsonObject = match jsb.get_content("jsonbank/sdk-test/index.json") {
        Ok(data) => data,
        Err(err) => panic!("{:?}", err)
    };

    println!("{:?}", data);
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