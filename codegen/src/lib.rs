#![forbid(
    unsafe_code,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_in_result,
    clippy::unwrap_used
)]

use core::str::FromStr;
use proc_macro::{TokenStream, TokenTree};

#[proc_macro]
pub fn named_colour(name_and_code: TokenStream) -> TokenStream {
    let mut tree = name_and_code.into_iter();
    let name = match tree.next() {
        Some(TokenTree::Ident(ident)) => ident,
        _ => panic!("expected identifier"),
    };
    let _comma = match tree.next() {
        Some(TokenTree::Punct(punct)) => punct,
        _ => panic!("expected comma"),
    };
    let code = match tree.next() {
        Some(TokenTree::Literal(literal)) => literal,
        _ => panic!("expected literal"),
    };
    let hex = &code.to_string()[2..];
    let ret = TokenStream::from_str(&format!(
        r#"
/// A colour with the hex code `#{hex}`
#[allow(clippy::unreadable_literal)]
pub const {name}: Self = Self::from_colour_code({code});"#
    ))
    .unwrap();
    ret
}
