use std::{collections::HashMap, fmt::Display, io::Error, panic};

use crate::{config::ConfigBlock, f64_from_string, read::get_file_contents, valid_hexadecimal};

pub mod map {
    use std::{collections::HashMap, hash::Hash};

    use crate::{
        Color,
        parse::{Entry, EntryKind},
    };

    pub struct DataMap {
        numbers: HashMap<String, f64>,
        texts: HashMap<String, String>,
        colors: HashMap<String, Color>,
        hexadecimal_colors: HashMap<String, String>,
        bools: HashMap<String, bool>,
    }

    impl DataMap {
        pub fn build(entries: Vec<Entry>)-> DataMap {
            let mut numbers: HashMap<String, f64> = HashMap::new();
            let mut texts: HashMap<String, String> = HashMap::new();
            let mut colors: HashMap<String, Color> = HashMap::new();
            let mut hexadecimal_colors: HashMap<String, String> = HashMap::new();
            let mut bools: HashMap<String, bool> = HashMap::new();

            for entry in entries {
                match entry.kind() {
                    EntryKind::Number(x) => {
                        numbers.insert(entry.name, x);
                    }
                    EntryKind::RGBColor(r, g, b) => {
                        colors.insert(entry.name, Color { r, g, b });
                    }
                    EntryKind::HexadecimalColor(x) => {
                        hexadecimal_colors.insert(entry.name, x);
                    }
                    EntryKind::Bool(x) => {
                        bools.insert(entry.name, x);
                    }
                    EntryKind::Text(x) => {
                        texts.insert(entry.name, x);
                    }
                };
            }

            DataMap { numbers, texts, colors, hexadecimal_colors, bools }
        }

        pub fn numbers(&self) -> &HashMap<String, f64> {
            &self.numbers
        }

        pub fn bools(&self) -> &HashMap<String, bool> {
            &self.bools
        }

        pub fn colors(&self) -> &HashMap<String, Color> {
            &self.colors
        }

        pub fn texts(&self) -> &HashMap<String, String> {
            &self.texts
        }

        pub fn hexadecimal_colors(&self) -> &HashMap<String, String> {
            &self.hexadecimal_colors
        }
    }
}

#[derive(Debug)]
pub struct Entry {
    name: String,
    kind: EntryKind,
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "
name : {},
kind : {},
content : {}",
            self.name,
            self.kind.kind(),
            self.kind.content()
        );
        Ok(())
    }
}

impl Entry {
    pub fn kind(&self) -> EntryKind {
        self.kind.clone()
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub enum EntryKind {
    Number(f64),
    Bool(bool),
    Text(String),
    RGBColor(u8, u8, u8),
    HexadecimalColor(String),
}

impl EntryKind {
    pub fn is_type(string: &str) -> bool {
        matches!(
            string.to_lowercase().as_str(),
            "color" | "number" | "text" | "bool"
        )
    }

    pub fn content(&self) -> String {
        match &self {
            EntryKind::RGBColor(r, g, b) => format!("({r}, {g}, {b})"),
            EntryKind::HexadecimalColor(c) => format!("{c}"),
            EntryKind::Text(s) => String::from(s),
            EntryKind::Number(f) => format!("{f}"),
            EntryKind::Bool(val) => format!("{val}"),
        }
    }

    pub fn kind(&self) -> &str {
        match self {
            EntryKind::Number(..) => "Number",
            EntryKind::Bool(..) => "Bool",
            EntryKind::RGBColor(..) => "Color",
            EntryKind::HexadecimalColor(..) => "Hexadecimal Color",
            EntryKind::Text(..) => "Text",
        }
    }

    pub fn build_from_string(kind: &String, value: &String) -> Self {
        if !Self::is_type(kind) {
            panic!("{kind} is not a valid type")
        }

        match kind.to_lowercase().as_str() {
            "color" => {
                if value.starts_with('#') {
                    if !valid_hexadecimal(value) {
                        panic!("Invalid hexadecimal !")
                    }
                    return EntryKind::HexadecimalColor(String::from(value));
                }
                let mut color: Vec<u8> = Vec::new();

                for val in value.split(|c: char| !c.is_numeric()) {
                    color.push(f64_from_string(val) as u8)
                }

                EntryKind::RGBColor(color[0], color[1], color[2])
            }

            "number" => EntryKind::Number(f64_from_string(value)),

            "bool" => {
                let lower = value.to_lowercase();
                if lower == "yes" || lower == "y" || lower == "true" {
                    EntryKind::Bool(true)
                } else if lower == "no" || lower == "n" || lower == "false" {
                    EntryKind::Bool(false)
                } else {
                    panic!("Invalid boolean !")
                }
            }

            text => EntryKind::Text(String::from(value)),

            _ => {
                panic!("Unknown Type !")
            }
        }
    }
}

fn build_text(content: ConfigBlock) -> String {
    if !content.is_valid() {
        panic!("{}", content.error_message())
    }
    content.file_content()
}

pub fn parse_file(path: String) -> Vec<Entry> {
    let block = get_file_contents(path);
    let file_content = build_text(block);

    let mut entries: Vec<Entry> = Vec::new();
    for line in file_content.lines() {
        let trimmed_line: String = line.chars().filter(|c| !c.is_whitespace()).collect();
        if trimmed_line.starts_with("?") {
            continue;
        }

        let mut kind: String = String::new();
        let mut name: String = String::new();
        let mut value: String = String::new();

        for element in trimmed_line.split(|c| c == ':' || c == '=') {
            if name.is_empty() {
                name = String::from(element);
                continue;
            } else if kind.is_empty() {
                kind = String::from(element);
                continue;
            } else if value.is_empty() {
                value = String::from(element);
                continue;
            } else {
                panic!("Too many arguments !");
            }
        }

        entries.push(Entry {
            name,
            kind: EntryKind::build_from_string(&kind, &value),
        });
    }

    entries
}
