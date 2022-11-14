# Oxd

**A Rust client for the Oxford Dictionary API**

## Usage

```rust
use oxd::{build_client, get_entry};

let client = build_cient(app_id, app_key);
let retrieve_entry = get_entry(&client, "rust");
```