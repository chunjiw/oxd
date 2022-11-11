use colored::Colorize;
use serde::Deserialize;
use serde_json::Value;

/// trait TerminalShow
pub trait StdoutDisplay {
    fn display(&self, prefix: &str);
}

impl StdoutDisplay for HeadwordEntry {
    fn display(&self, _prefix: &str) {
        println!("{}", self.word);
        self.lexical_entries.display("");
        println!();
    }
}

impl StdoutDisplay for LexicalEntry {
    fn display(&self, _prefix: &str) {
        print!("{} ", self.lexical_category.id.italic());
        self.entries.display("");
    }
}

impl StdoutDisplay for Entry {
    fn display(&self, _prefix: &str) {
        self.pronunciations.display("");
        println!();
        self.senses.display("  ");
    }
}

impl StdoutDisplay for Sense {
    fn display(&self, prefix: &str) {
        self.definitions.display(prefix);
        self.examples.display(prefix);
        self.subsenses.display("      ");
    }
}

/// trait StdoutDisplay works for Option<T> as long as the trait is implemented for T.
impl<T: StdoutDisplay> StdoutDisplay for Option<T> {
    fn display(&self, indent: &str) {
        if let Some(value) = &self {
            value.display(indent);
        }
    }
}

/// trait StdoutDisplay works for Vec<T> as long as the trait is implemented for T
impl<T: StdoutDisplay> StdoutDisplay for Vec<T> {
    fn display(&self, indent: &str) {
        for value in self {
            value.display(indent);
        }
    }
}

impl StdoutDisplay for String {
    fn display(&self, prefix: &str) {
        println!("{prefix}{self}");
    }
}

impl StdoutDisplay for Example {
    fn display(&self, prefix: &str) {
        let text = format!("\"{}\"", self.text.trim());
        println!("{}{}", prefix, text.italic().blue());
    }
}

impl StdoutDisplay for Pronunciation {
    fn display(&self, _prefix: &str) {
        if self.phonetic_notation == "IPA" {
            print!("/{}/ ", self.phonetic_spelling)
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
    pub definitions: Option<Vec<String>>,
    pub examples: Option<Vec<Example>>,
    pub subsenses: Option<Vec<Sense>>,
    pub domains: Option<Vec<Domain>>,
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

#[derive(Debug, Deserialize)]
pub struct Domain {
    pub id: String,
    pub text: String,
}
