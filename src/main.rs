use http_client::{get_def, Client};

fn main() {
    let client = Client::new();

    let body = get_def(client, "julep");

    println!("Body: {:#?}", body);

    println!("{}", body.results[0].word);
}
