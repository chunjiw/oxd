use colored::Colorize;
use oxd::models::*;
use std::fmt::Write;

pub trait Display {
    fn display(&self, canvas: &mut String);
}

impl Display for HeadwordEntry {
    fn display(&self, canvas: &mut String) {
        writeln!(canvas, "{}", self.word).unwrap();
        self.lexical_entries.display(canvas);
        writeln!(canvas).unwrap();
    }
}

impl Display for LexicalEntry {
    fn display(&self, canvas: &mut String) {
        write!(canvas, "{}  ", self.lexical_category.id.italic()).unwrap();
        self.entries.display(canvas);
    }
}

impl Display for Entry {
    fn display(&self, canvas: &mut String) {
        self.pronunciations.display(canvas);
        writeln!(canvas).unwrap();
        self.senses.display(canvas);
    }
}

impl Display for Sense {
    fn display(&self, canvas: &mut String) {
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
        write!(canvas, "{c}").unwrap();
    }
}

impl<T: Display> Display for Option<T> {
    fn display(&self, canvas: &mut String) {
        if let Some(value) = &self {
            value.display(canvas);
        }
    }
}

impl<T: Display> Display for Vec<T> {
    fn display(&self, canvas: &mut String) {
        for value in self {
            value.display(canvas);
        }
    }
}

impl Display for String {
    fn display(&self, canvas: &mut String) {
        writeln!(canvas, "{}", self).unwrap();
    }
}

impl Display for Example {
    fn display(&self, canvas: &mut String) {
        let text = format!("\"{}\"", self.text.trim());
        writeln!(canvas, "{}", text.italic().blue()).unwrap();
    }
}

impl Display for Pronunciation {
    fn display(&self, canvas: &mut String) {
        if self.phonetic_notation == "IPA" {
            write!(canvas, "/{}/ ", self.phonetic_spelling).unwrap();
        }
    }
}

impl Display for Domain {
    fn display(&self, canvas: &mut String) {
        write!(canvas, "[{}] ", self.text).unwrap();
    }
}

impl Display for Register {
    fn display(&self, canvas: &mut String) {
        write!(canvas, "[{}] ", self.text).unwrap();
    }
}
