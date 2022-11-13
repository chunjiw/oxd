use display::Display;
use oxd::{build_client, get_def};
use std::{env, process};

mod display;

fn main() {
    let app_id = env::var("OD_API_APP_ID").unwrap_or_else(|err| {
        println!("Problem reading Oxford Dictionary API credentials: {err}");
        process::exit(1);
    });
    let app_key = env::var("OD_API_APP_KEY").unwrap_or_else(|err| {
        println!("Problem reading Oxford Dictionary API credentials: {err}");
        process::exit(1);
    });

    let args: Vec<String> = env::args().collect();
    let client = build_client(app_id, app_key);

    let mut canvas = String::new();

    for word in &args[1..] {
        // println!("Looking up {}", word.to_lowercase());
        if let Some(retrieve_entry) = get_def(&client, &word) {
            retrieve_entry.headword_entries.display(&mut canvas);
        }
    }

    println!("{canvas}");
}
