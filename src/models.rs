/*! # A series of structs modeling OD API retrieve entries
 * Struct hierarchy:
 * - [_Sense_](Sense) { [_domains_](Domain), [_registers_](Register), _definitions_, _cross_reference_markers_, [_examples_](Example), [_subsenses_](Sense) }
 * -   ^
 * - [Entry] { [_pronunciations_](Pronunciation), [_variant_forms_](VariantForm), _origins_ }
 * -   ^
 * - [LexicalEntry] { text, language, [lexical_category](LexicalCategory), [_derivative_of_](DerivativeOf) }
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
    #[serde(rename = "etymologies")]
    pub origins: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct LexicalEntry {
    pub entries: Vec<Entry>,
    pub language: String,
    #[serde(rename = "lexicalCategory")]
    pub lexical_category: LexicalCategory,
    #[serde(rename = "derivativeOf")]
    pub derivative_of: Option<Vec<DerivativeOf>>,
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
    pub phonetic_spelling: Option<String>,
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

#[derive(Debug, Deserialize, Clone)]
pub struct DerivativeOf {
    pub id: String,
    pub text: String,
}

// Eq for Pronunciation
impl PartialEq for Pronunciation {
    fn eq(&self, other: &Self) -> bool {
        self.phonetic_notation == other.phonetic_notation
            && self.phonetic_spelling == other.phonetic_spelling
    }
}
impl Eq for Pronunciation {}

pub fn has_consistent_pronunciation(headword: &HeadwordEntry) -> bool {
    let lexical_entries = &headword.lexical_entries;
    let mut pronunciations: Vec<&Vec<Pronunciation>> = Vec::new();
    for lexical_entry in lexical_entries {
        for entry in &lexical_entry.entries {
            if let Some(p) = &entry.pronunciations {
                pronunciations.push(p);
            }
        }
    }
    return have_same_elements(pronunciations);
}

/// Returns whether two vectors have the same elements.
fn have_same_elements<T: Eq>(vv: Vec<&Vec<T>>) -> bool {
    if vv.len() <= 1 {
        return true;
    }
    for e1 in vv[0] {
        for v in &vv[1..] {
            if !v.contains(&e1) {
                return false;
            }
        }
    }
    return true;
}

#[test]
fn test_have_same_elements() {
    let v123 = vec![1, 2, 3];
    let v124 = vec![1, 2, 4];
    let vv1 = vec![&v123, &v123];
    let vv2 = vec![&v123];
    let vv3: Vec<&Vec<isize>> = vec![];
    let vv4 = vec![&v123, &v124];
    assert_eq!(have_same_elements(vv1), true);
    assert_eq!(have_same_elements(vv2), true);
    assert_eq!(have_same_elements(vv3), true);
    assert_eq!(have_same_elements(vv4), false);
}

// Helper functions to look at empty entries

pub fn is_empty_entries(entries: &Vec<Entry>) -> bool {
    return entries.iter().all(|entry| {
        entry.pronunciations.is_none() && entry.senses.is_none() && entry.variant_forms.is_none()
    });
}

pub fn is_empty_sense(sense: &Sense) -> bool {
    sense.definitions.is_none() && sense.cross_reference_markers.is_none()
}

pub fn roots(retrieve_entry: &RetrieveEntry) -> Vec<DerivativeOf> {
    let mut roots: Vec<DerivativeOf> = vec![];
    for headword in &retrieve_entry.headword_entries {
        for lexical_entry in &headword.lexical_entries {
            if let Some(derivative_ofs) = &lexical_entry.derivative_of {
                roots.append(&mut derivative_ofs.clone());
            }
        }
    }
    roots
}

impl HeadwordEntry {
    pub fn origins<'a>(&'a self) -> Vec<&'a String> {
        self.lexical_entries
            .iter()
            .flat_map(|lexical_entry| {
                lexical_entry
                    .entries
                    .iter()
                    .filter_map(|e| e.origins.as_ref())
                    .flatten()
            })
            .collect()
    }
}
