use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct RetrieveEntry {
    pub id: String,
    pub metadata: Value,
    pub results: Vec<HeadwordEntry>,
    pub word: String,
}

#[derive(Debug, Deserialize)]
pub struct LexicalCategory {
    pub id: String,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct LexicalEntry {
    pub language: String,
    #[serde(rename = "lexicalCategory")]
    pub lexical_category: LexicalCategory,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct HeadwordEntry {
    pub id: String,
    pub language: String,
    #[serde(rename = "lexicalEntries")]
    pub lexical_entries: Vec<LexicalEntry>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub word: String,
}
