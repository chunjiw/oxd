use oxd::display::Display;
use oxd::pronounce::Pronounce;
use oxd::{build_client, get_entry};
use std::{env, process};

mod args;
use args::OxdArgs;
use clap::Parser;

fn main() {
    let args = OxdArgs::parse();

    let app_id = env::var("OD_API_APP_ID").unwrap_or_else(|err| {
        eprintln!("Problem reading Oxford Dictionary API App ID: {err}\nGet one at https://developer.oxforddictionaries.com/ and set OD_API_APP_ID.");
        process::exit(1);
    });
    let app_key = env::var("OD_API_APP_KEY").unwrap_or_else(|err| {
        eprintln!("Problem reading Oxford Dictionary API App KEY: {err}\nGet one at https://developer.oxforddictionaries.com/ and set OD_API_APP_KEY.");
        process::exit(1);
    });

    let client = build_client(app_id, app_key);

    if let Some(retrieve_entry) = get_entry(&client, &args.word) {
        let mut canvas = String::new();
        retrieve_entry.display(&mut canvas);
        println!("{canvas}");
        if args.sound {
            retrieve_entry.pronounce();
        }
    }
}
