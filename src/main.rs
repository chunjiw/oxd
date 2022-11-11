use oxd::models::StdoutDisplay;
use oxd::{get_def, Client};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let client = Client::new();

    for word in &args[1..] {
        let retrieve_entry = get_def(&client, &word);
        retrieve_entry.headword_entries.display("");
    }
}
