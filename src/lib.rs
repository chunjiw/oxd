/*!
Oxd is a client library for the Oxford Dictionary API.
It provides [a series of structs](models) modeling entries returned from the API,
a function [get_entry] to get entries from the API,
a [Display](display::Display) trait to display entries,
and a [Pronounce](pronounce::Pronounce) trait to play pronunciation files.

# Usage

First you need to go to the [Oxford Dictionary API website](https://developer.oxforddictionaries.com/)
to get an application id and a key.

## Use as a library

```rust
use oxd::{build_client, get_entry};

let app_id = "your_app_id".to_owned();
let app_key = "your_app_key".to_owned();

let client = build_client(app_id, app_key);
if let Some(retrieve_entry) = get_entry(&client, "rust") {
    println!("{:#?}", retrieve_entry);
}

```

## Use as a command line utility

Currently the most convenient way to install oxd is via cargo:
```text
cargo install oxd
```
After installation, set environment variables `OD_API_APP_ID` and `OD_API_APP_KEY`
to their corresponding values obtained from
the [Oxford Dictionary API website](https://developer.oxforddictionaries.com/).
Then just type `oxd rust` to look up the word "rust".

![Screenshot](https://raw.githubusercontent.com/chunjiw/oxd/main/screenshot.png)
*/

use reqwest::header::{self, HeaderValue};
use reqwest::{blocking, StatusCode, Url};

pub mod display;
pub mod models;
pub mod pronounce;

const OD_API_BASE_URL: &str = "https://od-api.oxforddictionaries.com/api/v2/";

/// Builds a blocking client from OD API credentials.
pub fn build_client(app_id: String, app_key: String) -> blocking::Client {
    let expect = "Should be able to construct HeaderValue from String";
    let mut headers = header::HeaderMap::new();
    headers.insert("app_id", HeaderValue::from_str(&app_id).expect(expect));
    headers.insert("app_key", HeaderValue::from_str(&app_key).expect(expect));

    blocking::Client::builder()
        .default_headers(headers)
        .build()
        .expect("Should be able to build a client without issues")
}

fn build_full_url(word: &str) -> Url {
    let base = Url::parse(OD_API_BASE_URL).unwrap();
    let mut url = base.join("words/en-us").unwrap();
    url.query_pairs_mut().append_pair("q", word);
    // println!("{}", url);
    url
}

/// Querys the API and returns the results.
pub fn get_entry(client: &blocking::Client, word: &str) -> Option<models::RetrieveEntry> {
    let full_url = build_full_url(word);
    let res = client.get(full_url).send().unwrap();
    if res.status() != StatusCode::OK {
        eprintln!("Get {} from OD API when querying {}", res.status(), word);
        return None;
    }
    let body: models::RetrieveEntry = serde_json::from_reader(res).unwrap();
    Some(body)
}
