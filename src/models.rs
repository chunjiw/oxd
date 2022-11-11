use serde::Deserialize;
use serde_json::Value;

/// trait TerminalShow
pub trait TerminalShow {
    fn show(&self, indent: bool);
}

impl TerminalShow for LexicalEntry {
    fn show(&self, _indent: bool) {
        print!("{} ", self.lexical_category.id);
        for entry in &self.entries {
            entry.show(false);
        }
    }
}

impl TerminalShow for Entry {
    fn show(&self, _indent: bool) {
        for pronunciation in &self.pronunciations {
            if pronunciation.phonetic_notation == "IPA" {
                println!("/{}/", pronunciation.phonetic_spelling)
            }
        }
        for sense in &self.senses {
            sense.show(false);
        }
    }
}

impl TerminalShow for Sense {
    fn show(&self, indent: bool) {
        let prefix = if indent { "      " } else { "  " };
        for definition in &self.definitions {
            print!("{prefix}");
            println!("{}", definition);
        }
        if let Some(examples) = &self.examples {
            for example in examples {
                print!("{prefix}");
                println!("\"{}\"", example.text);
            }
        }
        if let Some(subsenses) = &self.subsenses {
            for subsense in subsenses {
                subsense.show(true);
            }
        }
    }
}

/*
Sense { [definitions], [examples] }
  ^
Entry { [Pronunciation] }
  ^
LexicalEntry { text, language, LexicalCategory }
  ^
HeadwordEntry { id, word, type, language }
  ^
RetrieveEntry { id, word, metadata }
*/

#[derive(Debug, Deserialize)]
pub struct Sense {
    pub definitions: Vec<String>,
    pub examples: Option<Vec<Example>>,
    pub subsenses: Option<Vec<Sense>>,
}

#[derive(Debug, Deserialize)]
pub struct Entry {
    pub senses: Vec<Sense>,
    pub pronunciations: Vec<Pronunciation>,
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
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct Pronunciation {
    #[serde(rename = "phoneticSpelling")]
    pub phonetic_spelling: String,
    #[serde(rename = "phoneticNotation")]
    pub phonetic_notation: String,
}
