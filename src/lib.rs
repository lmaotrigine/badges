#![forbid(unsafe_code)]
#![deny(clippy::pedantic, clippy::nursery, missing_docs)]

//! A library for generating SVG badges.
//!
//! This is a port of the [shields.io](https://shields.io) badge generation code to Rust.
//!
//! This library is a pretty opinionated port, and doesn't support some of the features of the
//! original library like badge styles, built-in icons, etc. It will only be updated to support
//! features that I need for my own projects, so if you need something more full-featured, you
//! may be better off looking elsewhere.
//!
//! # Example
//!
//! ```rust
//! use badges::{Badge, Colour, Render};
//! let badge = Badge::builder().label("hello").message("world").colour(Colour::GREEN).build();
//! println!("{}", badge.render());
//! ```
mod badge;
mod colour;
mod font;
mod vendor;
mod xml;

const FONT_FAMILY: &str = "Verdana,Geneva,DejaVu Sans,sans-serif";
const FONT_SCALE_UP_FACTOR: f32 = 10.0;
const FONT_SCALE_DOWN_VALUE: &str = "scale(.1)";
const BRIGHTNESS_THRESHOLD: u8 = 176; // 0.69 * 255
const HEIGHT: f32 = 20.0;
const VERTICAL_MARGIN: f32 = 0.0;
const SHADOW: bool = true;
const LOGO_HEIGHT: f32 = 14.0;

pub use badge::{Badge, Builder as BadgeBuilder};
pub use colour::Colour;
pub use xml::Render;
