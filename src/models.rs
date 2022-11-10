use serde::Deserialize;
use serde_json::Value;
use std::fmt;

// impl Display

impl fmt::Display for LexicalEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = writeln!(f, "{}", self.lexical_category.text);
        write!(f, "{}", self.entries[0].senses[0])
    }
}

impl fmt::Display for Sense {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for definition in &self.definitions {
            writeln!(f, "{}", definition)?;
        }
        for example in &self.examples {
            writeln!(f, "\"{}\"", example.text)?;
        }
        if let Some(subsenses) = &self.subsenses {
            for subsense in subsenses {
                writeln!(f, "    {}", subsense)?;
            }
        }
        Ok(())
    }
}

// Sense <- Entry <- LexicalEntry <- HeadwordEntry <- RetrieveEntry

#[derive(Debug, Deserialize)]
pub struct Sense {
    pub definitions: Vec<String>,
    pub examples: Vec<Example>,
    pub subsenses: Option<Vec<Sense>>,
}

#[derive(Debug, Deserialize)]
pub struct Entry {
    pub senses: Vec<Sense>,
}

#[derive(Debug, Deserialize)]
pub struct LexicalEntry {
    pub entries: Vec<Entry>,
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
    pub type_: String,
    pub word: String,
}

#[derive(Debug, Deserialize)]
pub struct RetrieveEntry {
    pub id: String,
    pub metadata: Value,
    #[serde(rename = "results")]
    pub headword_entries: Vec<HeadwordEntry>,
    pub word: String,
}

// Structs that have no struct fields

#[derive(Debug, Deserialize)]
pub struct LexicalCategory {
    pub id: String,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct Example {
    pub text: String
}