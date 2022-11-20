/*! Display entries as terminal text or html */

use crate::models::*;
use colored::Colorize;
use std::fmt::Write;

/// A trait to display entries
pub trait Display {
    fn display(&self, output: &mut String);
    fn to_html(&self, output: &mut String);
}

impl Display for RetrieveEntry {
    fn display(&self, output: &mut String) {
        for headword in &self.headword_entries {
            let mut c1 = String::new();
            headword.display(&mut c1);
            output.push_str(&c1);
        }
    }
    fn to_html(&self, output: &mut String) {
        for headword in &self.headword_entries {
            let mut c1 = String::new();
            headword.to_html(&mut c1);
            output.push_str(&c1);
        }
    }
}

impl Display for HeadwordEntry {
    fn display(&self, output: &mut String) {
        write!(output, "{}  ", self.word).unwrap();
        if has_consistent_pronunciation(&self) {
            // Assume "at least one `LexicalEntry`" and "must only one `Entry`"
            self.lexical_entries[0].entries[0]
                .pronunciations
                .display(output);
        }
        writeln!(output).unwrap();
        self.lexical_entries.display(output);
        writeln!(output).unwrap();
    }
    fn to_html(&self, output: &mut String) {
        write!(output, "<p>{}  ", self.word).unwrap();
        if has_consistent_pronunciation(&self) {
            // Assume "at least one `LexicalEntry`" and "must only one `Entry`"
            self.lexical_entries[0].entries[0]
                .pronunciations
                .to_html(output);
        }
        write!(output, "</p>").unwrap();
        self.lexical_entries.to_html(output);
    }
}

impl Display for LexicalEntry {
    fn display(&self, output: &mut String) {
        if is_empty_entries(&self.entries) {
            return;
        }
        write!(output, "\n{}  ", self.lexical_category.id.italic()).unwrap();
        self.entries.display(output);
    }
    fn to_html(&self, output: &mut String) {
        if is_empty_entries(&self.entries) {
            return;
        }
        write!(output, "<p><i>{}</i>  ", self.lexical_category.id).unwrap();
        self.entries.to_html(output);
    }
}

impl Display for Entry {
    fn display(&self, output: &mut String) {
        let mut lines = output.lines();
        if let Some(head) = lines.nth(0) {
            if !head.contains('/') {
                self.pronunciations.display(output);
            }
        }
        self.variant_forms.display(output);
        writeln!(output).unwrap();
        self.senses.display(output);
    }
    fn to_html(&self, output: &mut String) {
        let mut lines = output.split("</p>");
        if let Some(head) = lines.nth(0) {
            if !head.contains('/') {
                self.pronunciations.display(output);
            }
        }
        self.variant_forms.to_html(output);
        // Assume "must only one `Entry` to close <p> tag"
        write!(output, "</p><ul>").unwrap();
        self.senses.to_html(output);
        write!(output, "</ul>").unwrap();
    }
}

impl Display for Sense {
    fn display(&self, output: &mut String) {
        if is_empty_sense(self) {
            return;
        }
        let mut c = String::new();
        self.domains.display(&mut c);
        self.registers.display(&mut c);
        if self.domains.is_some() || self.registers.is_some() {
            writeln!(&mut c).unwrap();
        }
        self.definitions.display(&mut c);
        self.cross_reference_markers.display(&mut c);
        self.examples.display(&mut c);
        self.subsenses.display(&mut c);
        c = c.replace("\n", "\n  ");
        c.pop();
        c.pop();
        c.insert_str(0, "- ");
        write!(output, "{c}").unwrap();
    }
    fn to_html(&self, output: &mut String) {
        if is_empty_sense(self) {
            return;
        }
        write!(output, "<li>").unwrap();
        self.domains.to_html(output);
        self.registers.to_html(output);
        if self.domains.is_some() || self.registers.is_some() {
            write!(output, "<br>").unwrap();
        }
        self.definitions.to_html(output);
        self.cross_reference_markers.to_html(output);
        self.examples.to_html(output);
        write!(output, "<ul>").unwrap();
        self.subsenses.to_html(output);
        write!(output, "</ul></li>").unwrap();
    }
}

impl<T: Display> Display for Option<T> {
    fn display(&self, output: &mut String) {
        if let Some(value) = &self {
            value.display(output);
        }
    }
    fn to_html(&self, output: &mut String) {
        if let Some(value) = &self {
            value.to_html(output);
        }
    }
}

impl<T: Display> Display for Vec<T> {
    fn display(&self, output: &mut String) {
        for value in self {
            value.display(output);
        }
    }
    fn to_html(&self, output: &mut String) {
        for value in self {
            value.to_html(output);
        }
    }
}

impl Display for String {
    fn display(&self, output: &mut String) {
        write!(output, "{}", self).unwrap();
        if !self.ends_with('.') {
            write!(output, ".").unwrap();
        }
        writeln!(output, "").unwrap();
    }
    fn to_html(&self, output: &mut String) {
        write!(output, "{}", self).unwrap();
        if !self.ends_with('.') {
            write!(output, ".").unwrap();
        }
        write!(output, "<br>").unwrap();
    }
}

impl Display for Example {
    fn display(&self, output: &mut String) {
        let text = format!("\"{}\"", self.text.trim());
        writeln!(output, "{}", text.italic().blue()).unwrap();
    }
    fn to_html(&self, output: &mut String) {
        let text = format!("\"{}\"", self.text.trim());
        write!(output, "{}<br>", text).unwrap();
    }
}

impl Display for VariantForm {
    fn display(&self, output: &mut String) {
        let region = match &self.regions {
            Some(regions) => format!(" [{}]", regions[0].text),
            None => String::new(),
        };
        write!(output, " (also {}{}) ", self.text, region).unwrap();
    }
    fn to_html(&self, output: &mut String) {
        let region = match &self.regions {
            Some(regions) => format!(" [{}]", regions[0].text),
            None => String::new(),
        };
        write!(output, " (also {}{}) ", self.text, region).unwrap();
    }
}

impl Display for Pronunciation {
    fn display(&self, output: &mut String) {
        if self.phonetic_notation == "IPA" {
            if let Some(spelling) = &self.phonetic_spelling {
                write!(output, "/{}/ ", spelling).unwrap();
            }
        }
    }
    fn to_html(&self, output: &mut String) {
        if self.phonetic_notation == "IPA" {
            if let Some(spelling) = &self.phonetic_spelling {
                write!(output, "/{}/ ", spelling).unwrap();
            }
        }
    }
}

impl Display for Domain {
    fn display(&self, output: &mut String) {
        write!(output, "[{}] ", self.text).unwrap();
    }
    fn to_html(&self, output: &mut String) {
        write!(output, "[{}] ", self.text).unwrap();
    }
}

impl Display for Register {
    fn display(&self, output: &mut String) {
        write!(output, "[{}] ", self.text).unwrap();
    }
    fn to_html(&self, output: &mut String) {
        write!(output, "[{}] ", self.text).unwrap();
    }
}
