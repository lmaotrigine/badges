use codegen::named_colour;
use core::fmt::Display;
#[cfg(feature = "hex_colours")]
use core::str::FromStr;
#[cfg(any(test, feature = "hex_colours"))]
use regex::Regex;

#[allow(clippy::inline_always)]
#[inline(always)]
const fn scale8(x: u8, scale: u8) -> u8 {
    (((x as u16) * (1u16 + scale as u16)) >> 8) as u8
}

/// An RGB colour.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Colour {
    r: u8,
    g: u8,
    b: u8,
}

impl From<(u8, u8, u8)> for Colour {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self { r, g, b }
    }
}

impl From<[u8; 3]> for Colour {
    fn from([r, g, b]: [u8; 3]) -> Self {
        Self { r, g, b }
    }
}

impl Colour {
    named_colour!(ALICE_BLUE, 0xf0f8ff);
    named_colour!(AMETHYST, 0x9966cc);
    named_colour!(ANTIQUE_WHITE, 0xfaebd7);
    named_colour!(AQUA, 0x00ffff);
    named_colour!(AQUAMARINE, 0x7fffd4);
    named_colour!(AZURE, 0xf0ffff);
    named_colour!(BEIGE, 0xf5f5dc);
    named_colour!(BISQUE, 0xffe4c4);
    named_colour!(BLACK, 0x000000);
    named_colour!(BLANCHED_ALMOND, 0xffebcd);
    named_colour!(BLUE, 0x0000ff);
    named_colour!(BLUE_VIOLET, 0x8a2be2);
    named_colour!(BROWN, 0xa52a2a);
    named_colour!(BURLY_WOOD, 0xdeb887);
    named_colour!(CADET_BLUE, 0x5f9ea0);
    named_colour!(CHARTREUSE, 0x7fff00);
    named_colour!(CHOCOLATE, 0xd2691e);
    named_colour!(CORAL, 0xff7f50);
    named_colour!(CORNFLOWER_BLUE, 0x6495ed);
    named_colour!(CORNSILK, 0xfff8dc);
    named_colour!(CRIMSON, 0xdc143c);
    named_colour!(CYAN, 0x00ffff);
    named_colour!(DARK_BLUE, 0x00008b);
    named_colour!(DARK_CYAN, 0x008b8b);
    named_colour!(DARK_GOLDENROD, 0xb8860b);
    named_colour!(DARK_GRAY, 0xa9a9a9);
    named_colour!(DARK_GREY, 0xa9a9a9);
    named_colour!(DARK_GREEN, 0x006400);
    named_colour!(DARK_KHAKI, 0xbdb76b);
    named_colour!(DARK_MAGENTA, 0x8b008b);
    named_colour!(DARK_OLIVE_GREEN, 0x556b2f);
    named_colour!(DARK_ORANGE, 0xff8c00);
    named_colour!(DARK_ORCHID, 0x9932cc);
    named_colour!(DARK_RED, 0x8b0000);
    named_colour!(DARK_SALMON, 0xe9967a);
    named_colour!(DARK_SEA_GREEN, 0x8fbc8f);
    named_colour!(DARK_SLATE_BLUE, 0x483d8b);
    named_colour!(DARK_SLATE_GRAY, 0x2f4f4f);
    named_colour!(DARK_SLATE_GREY, 0x2f4f4f);
    named_colour!(DARK_TURQUOISE, 0x00ced1);
    named_colour!(DARK_VIOLET, 0x9400d3);
    named_colour!(DEEP_PINK, 0xff1493);
    named_colour!(DEEP_SKY_BLUE, 0x00bfff);
    named_colour!(DIM_GRAY, 0x696969);
    named_colour!(DIM_GREY, 0x696969);
    named_colour!(DODGER_BLUE, 0x1e90ff);
    named_colour!(FIREBRICK, 0xb22222);
    named_colour!(FLORAL_WHITE, 0xfffaf0);
    named_colour!(FOREST_GREEN, 0x228b22);
    named_colour!(FUCHSIA, 0xff00ff);
    named_colour!(GAINSBORO, 0xdcdcdc);
    named_colour!(GHOST_WHITE, 0xf8f8ff);
    named_colour!(GOLD, 0xffd700);
    named_colour!(GOLDENROD, 0xdaa520);
    named_colour!(GRAY, 0x808080);
    named_colour!(GREY, 0x808080);
    named_colour!(GREEN, 0x008000);
    named_colour!(GREEN_YELLOW, 0xadff2f);
    named_colour!(HONEYDEW, 0xf0fff0);
    named_colour!(HOT_PINK, 0xff69b4);
    named_colour!(INDIAN_RED, 0xcd5c5c);
    named_colour!(INDIGO, 0x4b0082);
    named_colour!(IVORY, 0xfffff0);
    named_colour!(KHAKI, 0xf0e68c);
    named_colour!(LAVENDER, 0xe6e6fa);
    named_colour!(LAVENDER_BLUSH, 0xfff0f5);
    named_colour!(LAWN_GREEN, 0x7cfc00);
    named_colour!(LEMON_CHIFFON, 0xfffacd);
    named_colour!(LIGHT_BLUE, 0xadd8e6);
    named_colour!(LIGHT_CORAL, 0xf08080);
    named_colour!(LIGHT_CYAN, 0xe0ffff);
    named_colour!(LIGHT_GOLDENROD_YELLOW, 0xfafad2);
    named_colour!(LIGHT_GRAY, 0xd3d3d3);
    named_colour!(LIGHT_GREY, 0xd3d3d3);
    named_colour!(LIGHT_GREEN, 0x90ee90);
    named_colour!(LIGHT_PINK, 0xffb6c1);
    named_colour!(LIGHT_SALMON, 0xffa07a);
    named_colour!(LIGHT_SEA_GREEN, 0x20b2aa);
    named_colour!(LIGHT_SKY_BLUE, 0x87cefa);
    named_colour!(LIGHT_SLATE_GRAY, 0x778899);
    named_colour!(LIGHT_SLATE_GREY, 0x778899);
    named_colour!(LIGHT_STEEL_BLUE, 0xb0c4de);
    named_colour!(LIGHT_YELLOW, 0xffffe0);
    named_colour!(LIME, 0x00ff00);
    named_colour!(LIME_GREEN, 0x32cd32);
    named_colour!(LINEN, 0xfaf0e6);
    named_colour!(MAGENTA, 0xff00ff);
    named_colour!(MAROON, 0x800000);
    named_colour!(MEDIUM_AQUAMARINE, 0x66cdaa);
    named_colour!(MEDIUM_BLUE, 0x0000cd);
    named_colour!(MEDIUM_ORCHID, 0xba55d3);
    named_colour!(MEDIUM_PURPLE, 0x9370db);
    named_colour!(MEDIUM_SEA_GREEN, 0x3cb371);
    named_colour!(MEDIUM_SLATE_BLUE, 0x7b68ee);
    named_colour!(MEDIUM_SPRING_GREEN, 0x00fa9a);
    named_colour!(MEDIUM_TURQUOISE, 0x48d1cc);
    named_colour!(MEDIUM_VIOLET_RED, 0xc71585);
    named_colour!(MIDNIGHT_BLUE, 0x191970);
    named_colour!(MINT_CREAM, 0xf5fffa);
    named_colour!(MISTY_ROSE, 0xffe4e1);
    named_colour!(MOCCASIN, 0xffe4b5);
    named_colour!(NAVAJO_WHITE, 0xffdead);
    named_colour!(NAVY, 0x000080);
    named_colour!(OLD_LACE, 0xfdf5e6);
    named_colour!(OLIVE, 0x808000);
    named_colour!(OLIVE_DRAB, 0x6b8e23);
    named_colour!(ORANGE, 0xffa500);
    named_colour!(ORANGE_RED, 0xff4500);
    named_colour!(ORCHID, 0xda70d6);
    named_colour!(PALE_GOLDENROD, 0xeee8aa);
    named_colour!(PALE_GREEN, 0x98fb98);
    named_colour!(PALE_TURQUOISE, 0xafeeee);
    named_colour!(PALE_VIOLET_RED, 0xdb7093);
    named_colour!(PAPAYA_WHIP, 0xffefd5);
    named_colour!(PEACH_PUFF, 0xffdab9);
    named_colour!(PERU, 0xcd853f);
    named_colour!(PINK, 0xffc0cb);
    named_colour!(PLUM, 0xdda0dd);
    named_colour!(POWDER_BLUE, 0xb0e0e6);
    named_colour!(PURPLE, 0x800080);
    named_colour!(REBECCA_PURPLE, 0x663399);
    named_colour!(RED, 0xff0000);
    named_colour!(ROSY_BROWN, 0xbc8f8f);
    named_colour!(ROYAL_BLUE, 0x4169e1);
    named_colour!(SADDLE_BROWN, 0x8b4513);
    named_colour!(SALMON, 0xfa8072);
    named_colour!(SANDY_BROWN, 0xf4a460);
    named_colour!(SEA_GREEN, 0x2e8b57);
    named_colour!(SEA_SHELL, 0xfff5ee);
    named_colour!(SIENNA, 0xa0522d);
    named_colour!(SILVER, 0xc0c0c0);
    named_colour!(SKY_BLUE, 0x87ceeb);
    named_colour!(SLATE_BLUE, 0x6a5acd);
    named_colour!(SLATE_GRAY, 0x708090);
    named_colour!(SLATE_GREY, 0x708090);
    named_colour!(SNOW, 0xfffafa);
    named_colour!(SPRING_GREEN, 0x00ff7f);
    named_colour!(STEEL_BLUE, 0x4682b4);
    named_colour!(TAN, 0xd2b48c);
    named_colour!(TEAL, 0x008080);
    named_colour!(THISTLE, 0xd8bfd8);
    named_colour!(TOMATO, 0xff6347);
    named_colour!(TURQUOISE, 0x40e0d0);
    named_colour!(VIOLET, 0xee82ee);
    named_colour!(WHEAT, 0xf5deb3);
    named_colour!(WHITE, 0xffffff);
    named_colour!(WHITE_SMOKE, 0xf5f5f5);
    named_colour!(YELLOW, 0xffff00);
    named_colour!(YELLOW_GREEN, 0x9acd32);

    /// Create a new colour from an integer representation of an RGB colour.
    #[allow(clippy::cast_possible_truncation)]
    #[must_use]
    pub const fn from_colour_code(code: u32) -> Self {
        Self {
            r: (code >> 16) as u8,
            g: (code >> 8) as u8,
            b: code as u8,
        }
    }

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
    #[cfg(any(test, feature = "hex_colours"))]
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
    /// If this value is more than 176, the colour is considered "light", and the text colour
    /// will be charcoal. Otherwise, the text colour will be white.
    //#[allow(clippy::trivially_copy_pass_by_ref)] // we don't want to take ownership of self?
    #[must_use]
    pub const fn brightness(self) -> u8 {
        let mut luma: u8 = 0;
        luma += scale8(self.r, 54);
        luma += scale8(self.g, 183);
        luma += scale8(self.b, 18);
        luma
    }
}

impl Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

#[cfg(feature = "hex_colours")]
impl FromStr for Colour {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_hex(s).map_or_else(
            || {
                Err(format!(
                    r#"{s} is not a valid CSS hex string.
                    
To use named colours, make use of the constants provided by the `Colour` struct. e.g. Colour::WHITE"#
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
