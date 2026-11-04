//! The module that contains the [`Color`] struct
use hex::FromHexError;
use std::{fmt::Display};

use crate::error_handling::ColorParsingError;


/// This represents an RGB color, stored as it's individual color values and it's hexadecimal string
#[derive(Clone, Copy, std::cmp::PartialEq, Debug, Hash)]
pub enum Color {
    RGB(u8, u8, u8),
    RGBA(u8, u8, u8, u8),
    PALETTE(u8),
}


const TRANSPARENT: Color = Color::RGBA(0, 0, 0, 0);

impl Default for Color {
    fn default() -> Self {
        Color::rgb(0, 0, 0)
    }
}

impl Color {
    /// Create a new [`Color`] from a red color value, a green color value and a blue color
    /// value
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color::RGB(r, g, b)
    }

    /// Create a new [`Color`] from a red color value, a green color value, a blue color
    /// value and alpha value
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color::RGBA(r, g, b, a)
    }

    pub const fn palette(index: u8) -> Self {
        Color::PALETTE(index)
    }

    /// Create a new palette [`Color`] from a name.
    /// This will result in an error if the color name isn't recognized
    pub fn from_color_string(string: &str) -> Result<Self, ColorParsingError<'_>> {
        let parsed: String = string.chars().filter(|c| !c.is_whitespace()).collect();
        match parsed.to_lowercase().as_str() {
            "black" => Ok(Color::PALETTE(0)),
            "red" => Ok(Color::PALETTE(1)),
            "green" => Ok(Color::PALETTE(2)),
            "yellow" => Ok(Color::PALETTE(3)),
            "blue" => Ok(Color::PALETTE(4)),
            "magenta" => Ok(Color::PALETTE(5)),
            "cyan" => Ok(Color::PALETTE(6)),
            "white" => Ok(Color::PALETTE(7)),
            "brightblack" => Ok(Color::PALETTE(8)),
            "brightred" => Ok(Color::PALETTE(9)),
            "brightgreen" => Ok(Color::PALETTE(10)),
            "brightyellow" => Ok(Color::PALETTE(11)),
            "brightblue" => Ok(Color::PALETTE(12)),
            "brightmagenta" => Ok(Color::PALETTE(13)),
            "brightcyan" => Ok(Color::PALETTE(14)),
            "brightwhite" => Ok(Color::PALETTE(15)),
            "transparent" | "none" => Ok(TRANSPARENT),
            _ => Err(ColorParsingError::UnknownName(string)),
        }
    }

    /// Create a new [`Color`] from a string slice formatted this way :
    /// "(r,g,b,a)" (where alpha is optional)
    ///     - "()" are necessary
    ///     - Each value *must* be seperated by a comma
    ///     - Each value *must* be at most 255
    ///     - whitespace is irrelevant
    /// This will result in an error if the string cannot be parsed
    pub fn from_rgb_string(string: &str) -> Result<Self, ColorParsingError<'_>> {
        let mut decode: Vec<u8> = Vec::new();
        for color in string.split(',').filter(|s| !s.is_empty()) {
            let mut num: u16 = 0;
            for character in color.chars() {
                if character == '(' || character == ')' {
                    continue;
                }
                if !character.is_ascii_digit() {
                    return Err(ColorParsingError::NonNumericCharacter(string,character));
                }
                num *= 10;
                num += character as u16 - '0' as u16;
                if num > 255 {
                    return Err(ColorParsingError::Over255);
                }
            }
            decode.push(num as u8);
        }
        if decode.len() < 3 {
            return Err(ColorParsingError::InvalidArgumentCount {
                found: decode.len(),
            });
        }
        if decode.len() == 3 {
            Ok(Color::rgb(decode[0], decode[1], decode[2]))
        } else {
            Ok(Color::rgba(decode[0], decode[1], decode[2], decode[3]))
        }
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
        if decode.len() <= 2 || decode.len() > 4 {
            Err(FromHexError::InvalidStringLength)
        } else if decode.len() == 3 {
            Ok(Color::rgb(decode[0], decode[1], decode[2]))
        } else {
            Ok(Color::rgba(decode[0], decode[1], decode[2], decode[3]))
        }
    }

    /// Returns the alpha of a color, for values with no alpha, 255 will be returned
    pub const fn alpha(&self) -> u8 {
        match self {
            Color::RGB(_, _, _) => 255,
            Color::RGBA(_, _, _, a) => *a,
            Color::PALETTE(_) => 255,
        }
    }

    /// Returns the hexadecimal value of the color.
    /// For palette colors, this will simply return the
    /// index in hexadecimal
    pub fn hexadecimal_value(&self) -> String {
        match self {
            Color::RGB(r, g, b) => hex::encode([*r, *g, *b]),
            Color::RGBA(r, g, b, a) => hex::encode([*r, *g, *b, *a]),
            Color::PALETTE(p) => hex::encode([*p]),
        }
    }

    /// Returns the inverted variant of the color.
    /// For palette colors, this does nothing
    pub const fn inverted(&self) -> Self {
        match *self {
            Color::RGB(r, g, b) => Color::RGB(255 - r, 255 - g, 255 - b),
            Color::RGBA(r, g, b, a) => Color::RGBA(255 - r, 255 - g, 255 - b, a),
            Color::PALETTE(i) => Color::PALETTE(i),
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::RGB(r, g, b) => write!(
                f,
                "r: {r}, g: {g}, b: {b}, hex: {}",
                self.hexadecimal_value()
            ),
            Color::RGBA(r, g, b, a) => write!(
                f,
                "r: {r}, g: {g}, b: {b}, a: {a}, hex: {}",
                self.hexadecimal_value()
            ),
            Color::PALETTE(p) => write!(f, "palette index : {p}"),
        }
    }
}
