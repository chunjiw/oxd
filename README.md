# oxd

## Oxd

Oxd is a client library for the Oxford Dictionary API.
It provides [a series of structs](models) modeling entries returned from the API,
a function [get_entry] to get entries from the API,
a [Display](display::Display) trait to display entries,
and a [Pronounce](pronounce::Pronounce) trait to play pronunciation files.

## Usage

First you need to go to the [Oxford Dictionary API website](https://developer.oxforddictionaries.com/)
to get an application id and a key.

```rust
use oxd::{build_client, get_entry};

let app_id = "your_app_id".to_owned();
let app_key = "your_app_key".to_owned();

let client = build_client(app_id, app_key);
let retrieve_entry = get_entry(&client, "rust");
```

License: MIT
