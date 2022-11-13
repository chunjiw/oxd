/*! # A series of structs modeling OD API retrieve entries
 * Struct hierarchy:
 * - [_Sense_](Sense) { [_domains_](Domain), [_registers_](Register), _definitions_, _cross_reference_markers_, [_examples_](Example), [_subsenses_](Sense) }
 * -   ^
 * - [Entry] { [_pronunciations_](Pronunciation), [_variant_forms_](VariantForm) }
 * -   ^
 * - [LexicalEntry] { text, language, [lexical_category](LexicalCategory) }
 * -   ^
 * - [HeadwordEntry] { id, word, type, language }
 * -   ^
 * - [RetrieveEntry] { _id_, _word_, metadata }
 *
 * Italic fields are optional.
 */

use serde::Deserialize;
use serde_json::Value;

// Structs

#[derive(Debug, Deserialize)]
pub struct Sense {
    pub definitions: Option<Vec<String>>,
    pub examples: Option<Vec<Example>>,
    pub subsenses: Option<Vec<Sense>>,
    pub domains: Option<Vec<Domain>>,
    #[serde(rename = "crossReferenceMarkers")]
    pub cross_reference_markers: Option<Vec<String>>,
    pub registers: Option<Vec<Register>>,
}

#[derive(Debug, Deserialize)]
pub struct VariantForm {
    pub regions: Option<Vec<Region>>,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct Entry {
    pub senses: Option<Vec<Sense>>,
    pub pronunciations: Option<Vec<Pronunciation>>,
    #[serde(rename = "variantForms")]
    pub variant_forms: Option<Vec<VariantForm>>,
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
    pub id: Option<String>,
    pub metadata: Value,
    #[serde(rename = "results")]
    pub headword_entries: Vec<HeadwordEntry>,
    pub word: Option<String>,
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
    #[serde(rename = "audioFile")]
    pub audio_file: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Domain {
    pub id: String,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct Register {
    pub id: String,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct Region {
    pub id: String,
    pub text: String,
}
