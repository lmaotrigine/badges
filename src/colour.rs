use regex::Regex;
use std::{fmt::Display, str::FromStr};

/// An RGB colour.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Colour {
    r: u8,
    g: u8,
    b: u8,
}

impl Colour {
    /// A colour with the hex code `#333333`.
    pub const CHARCOAL: Self = Self::from_rgb(51, 51, 51);
    /// A colour with the hex code `#010101`.
    pub const BLACK: Self = Self::from_rgb(1, 1, 1);
    /// A colour with the hex code `#cccccc`.
    pub const SILVER: Self = Self::from_rgb(204, 204, 204);
    /// A colour with the hex code `#ffffff`.
    pub const WHITE: Self = Self::from_rgb(255, 255, 255);
    /// A colour with the hex code `#555555`.
    pub const GREY: Self = Self::from_rgb(85, 85, 85);
    /// A colour with the hex code `#44cc11`.
    pub const GREEN: Self = Self::from_rgb(68, 204, 17);
    /// A colour with the hex code `#e05d44`.
    pub const RED: Self = Self::from_rgb(224, 93, 68);

    /// Create a colour from a hex code.
    ///
    /// This function supports both long and short hex codes. Alpha values are not supported.
    ///
    /// If the hex code is invalid, this function will return `None`.
    ///
    /// # Panics
    ///
    /// If this function panics, it indicates that the Regex used to find long and short hex codes
    /// is invalid. If you encounter this, please file a bug report.
    #[must_use]
    pub fn from_hex(hex: &str) -> Option<Self> {
        let long_hex = Regex::new(r"^#([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})$").unwrap();
        let short_hex = Regex::new(r"^#([0-9a-fA-F])([0-9a-fA-F])([0-9a-fA-F])$").unwrap();
        if let Some(caps) = short_hex.captures(hex) {
            let mut it = caps.iter();
            it.next()?;
            let mut rgb = it.map(|cap| {
                Some(
                    u8::from_str_radix(cap?.as_str().repeat(2).as_str(), 16)
                        .expect("Invalid colour component not caught by regex"),
                )
            });
            return Some(Self {
                r: rgb.next()??,
                g: rgb.next()??,
                b: rgb.next()??,
            });
        }
        if let Some(caps) = long_hex.captures(hex) {
            let mut it = caps.iter();
            it.next()?;
            let mut rgb = it.map(|cap| {
                Some(
                    u8::from_str_radix(cap?.as_str(), 16)
                        .expect("Invalid colour component not caught by regex"),
                )
            });
            return Some(Self {
                r: rgb.next()??,
                g: rgb.next()??,
                b: rgb.next()??,
            });
        }
        None
    }

    /// Create a new colour from RGB values.
    #[must_use]
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Returns the "brightness" of the colour, as a value between 0 and 1.
    ///
    /// If this value is more than 0.69, the colour is considered "light", and the text colour
    /// will be charcoal. Otherwise, the text colour will be white.
    //#[allow(clippy::trivially_copy_pass_by_ref)] // we don't want to take ownership of self?
    #[must_use]
    pub fn brightness(self) -> f32 {
        f32::from(self.r).mul_add(
            299.0,
            f32::from(self.g).mul_add(507.0, f32::from(self.b) * 114.0),
        ) / 255_000.0
    }
}

impl Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

impl FromStr for Colour {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_hex(s).map_or_else(
            || {
                Err(format!(
                    r#"{s} is not a valid CSS colour

(named colours are not supported yet, please use hex codes)"#
                ))
            },
            Ok,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_hex() {
        assert_eq!(
            Colour::from_hex("#000000"),
            Some(Colour { r: 0, g: 0, b: 0 })
        );
        assert_eq!(Colour::from_hex("#ffffff"), Some(Colour::WHITE));
        assert_eq!(
            Colour::from_hex("#ff0000"),
            Some(Colour { r: 255, g: 0, b: 0 })
        );
        assert_eq!(
            Colour::from_hex("#00ff00"),
            Some(Colour { r: 0, g: 255, b: 0 })
        );
        assert_eq!(
            Colour::from_hex("#0000ff"),
            Some(Colour { r: 0, g: 0, b: 255 })
        );
        assert_eq!(Colour::from_hex("#000"), Some(Colour { r: 0, g: 0, b: 0 }));
        assert_eq!(Colour::from_hex("#fff"), Some(Colour::WHITE));
        assert_eq!(
            Colour::from_hex("#f00"),
            Some(Colour { r: 255, g: 0, b: 0 })
        );
        assert_eq!(
            Colour::from_hex("#0f0"),
            Some(Colour { r: 0, g: 255, b: 0 })
        );
        assert_eq!(
            Colour::from_hex("#00f"),
            Some(Colour { r: 0, g: 0, b: 255 })
        );
        assert_eq!(Colour::from_hex("#0000000"), None);
    }
}
