use oxd::models::StdoutDisplay;
use oxd::{get_def, Client};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let client = Client::new();

    for word in &args[1..] {
        let retrieve_entry = get_def(&client, &word);
        for headword_entry in retrieve_entry.headword_entries {
            println!("{}", headword_entry.word);
            for lexical_entry in headword_entry.lexical_entries {
                lexical_entry.display(false);
            }
        }
    }
}
