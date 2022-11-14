use reqwest::header::{self, HeaderValue};
use reqwest::{blocking, StatusCode, Url};

pub mod models;

const OD_API_BASE_URL: &str = "https://od-api.oxforddictionaries.com/api/v2/";

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

pub fn get_entry(client: &blocking::Client, word: &str) -> Option<models::RetrieveEntry> {
    let full_url = build_full_url(word);
    let res = client.get(full_url).send().unwrap();
    if res.status() != StatusCode::OK {
        eprintln!("Get {} from OD API when querying {}", res.status(), word);
        return None;
    }
    let body: models::RetrieveEntry = serde_json::from_reader(res).unwrap();
    // let mut body = String::new();
    // let _ = res.read_to_string(&mut body);

    // println!("Status: {}", res.status());
    // println!("Headers: {:#?}", res.headers());
    Some(body)
}
