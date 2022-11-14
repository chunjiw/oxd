use oxd::display::Display;
use oxd::pronounce::Pronounce;
use oxd::{build_client, get_entry};
use std::{env, process};

fn main() {
    let app_id = env::var("OD_API_APP_ID").unwrap_or_else(|err| {
        println!("Problem reading Oxford Dictionary API App ID: {err}");
        process::exit(1);
    });
    let app_key = env::var("OD_API_APP_KEY").unwrap_or_else(|err| {
        println!("Problem reading Oxford Dictionary API App KEY: {err}");
        process::exit(1);
    });

    let args: Vec<String> = env::args().collect();
    let client = build_client(app_id, app_key);

    for word in &args[1..] {
        if let Some(retrieve_entry) = get_entry(&client, &word) {
            let mut canvas = String::new();
            retrieve_entry.display(&mut canvas);
            println!("{canvas}");
            retrieve_entry.pronounce();
        }
    }
}
