# JsonBank.io Rust SDK

The official repository for the JsonBank.io Rust SDK.

[Documentation](https://docs.jsonbank.io/sdks/rust) | [crates.io](https://crates.io/crates/jsonbank) | [docs.rs](https://docs.rs/jsonbank/latest/jsonbank/)


## Usage

Add the following to your Cargo.toml file

```toml
[dependencies]
jsonbank = "0.1"
```

Then import the library in your code

```rust
use jsonbank::{JsonBank, JsonObject};

fn main() {
    let jsb = JsonBank::new_without_config();
    
    // get public content
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

or using npm because a package.json file is included

```bash
npm run test
```
