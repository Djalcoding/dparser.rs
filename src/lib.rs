pub mod parse;
pub mod read;


pub struct Color {
    r:u8,
    g:u8,
    b:u8,
}

pub fn f64_from_string(str: &str) -> f64 {
    let mut number: f64 = 0.0;
    let mut decimal: f64 = 1.0;
    let mut decimals: bool = false;
    for character in str.chars() {
        if !character.is_numeric() && character != '.' {
            panic!("Invalid character '{character}' for generating number");
        }
        if !decimals {
            if character == '.' {
                decimals = true;
                continue;
            }
        }
        else {
            decimal /= 10.0
        }
        let num = character as u32 - '0' as u32;
        if decimal == 1.0 {number *= 10.0;}
        number += num as f64 * decimal;
    }

    number
}

pub fn valid_hexadecimal(text: &str) -> bool{
    if text.len() != 7 {
        return false  
    }
    for character in text.chars() {
        if !character.is_ascii_hexdigit() && character != '#'{
            return false;  
        } 
    } 
    true
}

mod config {

    pub enum ConfigBlock {
        Valid(Config),
        Invalid(String),
    }

    impl ConfigBlock {
        pub fn new(content: String, path: String) -> Self {
            Self::Valid(Config { content, path })
        }

        pub fn invalid(error: String) -> Self {
            Self::Invalid(error)
        }

        pub fn is_valid(&self) -> bool {
            if let Self::Valid(_) = self {
                return true;
            }
            false
        }
        pub fn path(&self) -> String {
            if let Self::Valid(conf) = self {
                return conf.path.clone();
            }
            panic!("No Path Exists in ConfigBlock")
        }

        pub fn error_message(&self) -> String {
            if let Self::Invalid(err) = self {
                return err.clone();
            }
            panic!("Config Block is valid and has no error message")
        }

        pub fn file_content(&self) -> String {
            if let Self::Valid(config) = self {
                return config.get_content().clone();
            }
            panic!("Config Block has no content")
        }
    }

    pub struct Config {
        content: String,
        path: String,
    }

    impl Config {
        pub fn get_content(&self) -> &String {
            &self.content
        }
    }
}
