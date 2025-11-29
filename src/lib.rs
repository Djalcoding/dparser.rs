pub mod parse;
pub mod read;
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

    pub fn from_rgb_string(string: String) -> Self {
        let mut vec: Vec<u8> = Vec::new();
        for color in string
            .split(|c: char| !c.is_numeric())
            .filter(|s| !s.is_empty())
        {
            let mut num: u16 = 0;
            for character in color.chars() {
                assert!(num <= 255);
                num *= 10;
                num += character as u16 - '0' as u16;
            }
            vec.push(num as u8);
        }

        Color::rgb(vec[0], vec[1], vec[2])
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

    pub fn hexa(hexadecimal_color: &str) -> Self {
        let mut color: String = hexadecimal_color.to_string();
        if hexadecimal_color.starts_with("#") {
            color = hexadecimal_color.chars().filter(|c| *c != '#').collect();
        }
        let decode =
            hex::decode(&color).unwrap_or_else(|_| panic!("Invalid Hexadecimal ! {color}"));

        Color {
            r: decode[0],
            g: decode[1],
            b: decode[2],
            hexadecimal: String::from(hexadecimal_color),
        }
    }

    pub fn hexadecimal_value(&self) -> String {
        self.hexadecimal.clone()
    }
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
        } else {
            decimal /= 10.0
        }
        let num = character as u32 - '0' as u32;
        if decimal == 1.0 {
            number *= 10.0;
        }
        number += num as f64 * decimal;
    }

    number
}

pub fn valid_hexadecimal(text: &str) -> bool {
    if text.len() != 7 {
        return false;
    }
    for character in text.chars() {
        if !character.is_ascii_hexdigit() && character != '#' {
            return false;
        }
    }
    true
}
