//! The module that contains the [`Color`] struct 
use hex::FromHexError;
use std::{fmt::Display, io::Error};
/// This represents an RGB color, stored as it's individual color values and it's hexadecimal string
#[derive(Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    hexadecimal: String,
}

impl Color {
    /// Create a new [`Color`] from a red color value, a green color value and a blue color
    /// value
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color {
            r,
            g,
            b,
            hexadecimal: format!("#{r:02x}{g:02x}{b:02x}"),
        }
    }

    /// Create a new [`Color`] from a color name.
    /// This will result in an error if the color name isn't recognized
    /// ## recognized names
    ///  - red => (255, 0, 0)
    ///  - green => (0, 255, 0)
    ///  - blue => (0, 0, 255)
    ///  - white => (255, 255, 255)
    ///  - black => (0, 0, 0)
    ///  - yellow => (255, 255 ,0)
    ///  - purple => (255, 0, 255)
    ///  - cyan => (0, 255, 255)
    pub fn from_color_string(string: &str) -> Result<Self, Error> {
        match string.to_lowercase().as_str() {
            "red" => Ok(Color::rgb(255, 0, 0)),
            "green" => Ok(Color::rgb(0, 255, 0)),
            "blue" => Ok(Color::rgb(0, 0, 255)),
            "white" => Ok(Color::rgb(255, 255, 255)),
            "black" => Ok(Color::rgb(0, 0, 0)),
            "yellow" => Ok(Color::rgb(255, 255, 0)),
            "purple" => Ok(Color::rgb(255, 0, 255)),
            "cyan" => Ok(Color::rgb(0, 255, 255)),
            _ => Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("'{string}' is not a recognized color"),
            )),
        }
    }

    /// Create a new [`Color`] from a string slice formatted this way :
    /// "(r,g,b)"
    ///     - "()" are necessary
    ///     - Each value *must* be seperated by a comma
    ///     - Each value *must* be at most 255
    ///     - whitespace is irrelevant
    /// This will result in an error if the string cannot be parsed
    pub fn from_rgb_string(string: &str) -> Result<Self, Error> {
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
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "less than 3 arguments for color definition",
            ));
        }
        Ok(Color::rgb(decode[0], decode[1], decode[2]))
    }

    /// Create a new [`Color`] from a string slice representing an hexadecimal value
    ///     - The number of '#' at the start of the slice is irrelevant
    ///     - The slice must contain exactly 6 hexadecimal digits (ignoring '#')
    ///     - The casing of the hexadecimal value is irrelevant
    /// ## [API note]
    ///   - This makes use of the hex crate
    pub fn from_hexadecimal(hexadecimal_color: &str) -> Result<Self, FromHexError> {
        let mut color: String = hexadecimal_color.to_string();
        if hexadecimal_color.starts_with("#") {
            color = hexadecimal_color.chars().filter(|c| *c != '#').collect();
        }
        let decode = hex::decode(&color)?;

        Ok(Color::rgb(decode[0], decode[1], decode[2]))
    }

    /// Returns the red value of the color
    pub fn red(&self) -> u8 {
        self.r
    }

    /// Returns the green value of the color
    pub fn green(&self) -> u8 {
        self.g
    }

    /// Returns the blue value of the color
    pub fn blue(&self) -> u8 {
        self.b
    }

    /// Returns the hexadecimal value of the color
    pub fn hexadecimal_value(&self) -> String {
        self.hexadecimal.clone()
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "r: {}, g: {}, b: {}, hex: {}",
            self.red(),
            self.green(),
            self.blue(),
            self.hexadecimal_value()
        )
    }
}
