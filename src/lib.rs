use url::Url;

pub mod models;

const OD_API_BASE_URL: &str = "https://od-api.oxforddictionaries.com/api/v2/";
const OD_API_CLIENT_ID: &str = "5c40050a";
const OD_API_CLIENT_KEY: &str = "e5fccb756ea206da1ac164dd4fbb9d1e";

#[derive(Default)]
pub struct Client {
    http_client: reqwest::blocking::Client,
}

impl Client {
    pub fn new() -> Self {
        let http_client = reqwest::blocking::Client::new();
        Self { http_client }
    }
}

fn build_full_url(word: &str) -> Url {
    let base = Url::parse(OD_API_BASE_URL).unwrap();
    let u = base.join("entries/en-us/").unwrap();
    u.join(word).unwrap()
}

pub fn get_def(client: &Client, word: &str) -> models::RetrieveEntry {
    let full_url = build_full_url(word);
    // println!("{}", full_url);
    let res = client
        .http_client
        .get(full_url)
        .header("app_id", OD_API_CLIENT_ID)
        .header("app_key", OD_API_CLIENT_KEY)
        .query(&[("fields", "definitions,examples")])
        .send()
        .unwrap();
    let body: models::RetrieveEntry = serde_json::from_reader(res).unwrap();
    // let mut body = String::new();
    // let _ = res.read_to_string(&mut body);

    // println!("Status: {}", res.status());
    // println!("Headers: {:#?}", res.headers());
    body
}
