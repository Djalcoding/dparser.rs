pub mod parse;
mod read;

pub mod datastructure {
    use std::{collections::HashMap, io::Error};

    use crate::{color::Color, parse::{Entry, parse_file}};
    pub struct ParsedData {
        map: HashMap<String, String>,
    }

    impl ParsedData {
        pub fn from_file(path: &String) -> Result<Self, Error> {
            let entries: Vec<Entry> = parse_file(path)?;
            Ok(ParsedData::from_entries(entries))
        }

        fn from_entries(entries: Vec<Entry>) -> Self {
            let mut map: HashMap<String, String> = HashMap::new();

            for entry in entries {
                map.insert(String::from(entry.name()), String::from(entry.content()));
            }

            Self { map }
        }

        pub fn as_raw(&self, key: &String) -> Result<String, Error> {
            let modified_key = key.chars().filter(|c| c.is_alphanumeric()).collect::<String>().to_lowercase();
            let entry = self.map.get(&modified_key);
            if entry.is_none() {
                return Err(Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("The key {key} wasn't defined"),
                ));
            }
            Ok(String::from(entry.unwrap()))
        }

        pub fn as_text(&self, key: &String) -> Result<String, Error> {
            let raw = self.as_raw(key)?;
            let mut in_text: bool = false;
            let mut quote_count: u16 = 0;
            let s = raw
                .chars()
                .filter(|c| {
                    if *c == '"' {
                        quote_count += 1;
                        in_text = !in_text;
                        return false;
                    }
                    in_text
                })
                .collect();
            if quote_count > 2 {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "More than two quotation marks in text",
                ));
            }
            Ok(s)
        }

        pub fn as_number(&self, key: &String) -> Result<f64, Error> {
            let raw = self.as_raw(key)?;
            f64_from_string(&raw)
        }

        pub fn as_boolean(&self, key:&String) -> Result<bool, Error> {
            let raw = self.as_raw(key)?.to_lowercase();
            let yes:bool = raw == "yes" || raw == "y" || raw == "true" || raw == "1";
            let no:bool = raw == "no" || raw == "n" || raw == "false" || raw == "0";
            
            if !no && !yes {
                return Err(Error::new(std::io::ErrorKind::InvalidInput, "Invalide boolean symbol"));
            }
            Ok(yes)

        }

        pub fn as_color(&self, key:&String)-> Result<Color, Error> {
            let raw = self.as_raw(key)?;
            let rgb = Color::from_rgb_string(&raw);
            let hexadecimal = Color::from_hexadecimal(&raw);

            if let Ok(color) = rgb {
                return Ok(color);
            }
            else if let Ok(color) = hexadecimal {
                return Ok(color); 
            }
            else if let Err(color) = rgb {
                return Err(color);
            }
            
            Err(Error::new(std::io::ErrorKind::InvalidInput, hexadecimal.err().unwrap().to_string()))
        }
    }

    fn f64_from_string(str: &str) -> Result<f64, Error> {
        let mut number: f64 = 0.0;
        let mut decimal: f64 = 1.0;
        let mut decimals: bool = false;
        for character in str.chars() {
            if !character.is_numeric() && character != '.' {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Non numeric character in f64",
                ));
            }
            if !decimals {
                if character == '.' {
                    decimals = true;
                    continue;
                }
            } else {
                decimal /= 10.0
            }
            let num = character as u32 - '0' as u32;
            if decimal == 1.0 {
                number *= 10.0;
            }
            number += num as f64 * decimal;
        }

        Ok(number)
    }
}

mod color {
    use hex::FromHexError;
    use std::{fmt::Display, io::Error};

    #[derive(Clone)]
    pub struct Color {
        r: u8,
        g: u8,
        b: u8,
        hexadecimal: String,
    }

    impl Color {
        pub fn rgb(r: u8, g: u8, b: u8) -> Self {
            Color {
                r,
                g,
                b,
                hexadecimal: format!("#{r:02x}{g:02x}{b:02x}"),
            }
        }

        pub fn from_rgb_string(string: &String) -> Result<Self, Error> {
            let mut decode: Vec<u8> = Vec::new();
            for color in string
                .split(|c: char| !c.is_numeric())
                .filter(|s| !s.is_empty())
            {
                let mut num: u16 = 0;
                for character in color.chars() {
                    num *= 10;
                    num += character as u16 - '0' as u16;
                    if num > 255 {
                        return Err(Error::new(
                            std::io::ErrorKind::InvalidInput,
                            format!("Value over 255 in rgb : {num} at index {}", decode.len()),
                        ));
                    }
                }
                decode.push(num as u8);
            }
            if decode.len() < 3 {
                return Err(Error::new(std::io::ErrorKind::InvalidInput, "less than 3 arguments for color definition")) ;
            }
            Ok(Color::rgb(decode[0], decode[1], decode[2]))
        }

        pub fn from_hexadecimal(hexadecimal_color: &String) -> Result<Self, FromHexError> {
            let mut color: String = hexadecimal_color.to_string();
            if hexadecimal_color.starts_with("#") {
                color = hexadecimal_color.chars().filter(|c| *c != '#').collect();
            }
            let decode = hex::decode(&color)?;

            Ok(Color::rgb(decode[0], decode[1], decode[2]))
        }

        pub fn red(&self) -> u8 {
            self.r
        }

        pub fn green(&self) -> u8 {
            self.g
        }

        pub fn blue(&self) -> u8 {
            self.b
        }

        pub fn hexadecimal_value(&self) -> &String {
            &self.hexadecimal
        }
    }

    impl Display for Color {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f,"r: {}, g: {}, b: {}, hex: {}",self.red(),self.green(), self.blue(), self.hexadecimal_value()) 
        } 
    }
}
