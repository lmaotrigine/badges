use crate::traits::{IteratorExt, Render};
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct Attribute<'a> {
    name: &'a str,
    value: Cow<'a, str>,
}

impl std::fmt::Display for Attribute<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={:?}", self.name, self.value)
    }
}

fn escape_xml(s: &str) -> Cow<str> {
    let raw = s.as_bytes();
    let mut escaped = None;
    let mut iterator = raw.iter();
    let mut pos = 0;
    while let Some(i) = iterator.position(|b| matches!(*b, b'<' | b'>' | b'&' | b'"' | b'\'')) {
        if escaped.is_none() {
            escaped = Some(Vec::with_capacity(s.len()));
        }
        let escaped = escaped.as_mut().expect("Uninitialized vector");
        let new_pos = pos + i;
        escaped.extend_from_slice(&raw[pos..new_pos]);
        match raw[new_pos] {
            b'<' => escaped.extend_from_slice(b"&lt;"),
            b'>' => escaped.extend_from_slice(b"&gt;"),
            b'&' => escaped.extend_from_slice(b"&amp;"),
            b'"' => escaped.extend_from_slice(b"&quot;"),
            b'\'' => escaped.extend_from_slice(b"&apos;"),
            _ => unreachable!("Only <, >, &, \" and 'are escaped"),
        }
        pos = new_pos + 1;
    }
    escaped.map_or(Cow::Borrowed(s), |mut escaped| {
        if let Some(raw) = raw.get(pos..) {
            escaped.extend_from_slice(raw);
        }
        Cow::Owned(
            String::from_utf8(escaped).expect("Invalid UTF-8 inside a &str, should be impossible."),
        )
    })
}

#[derive(Debug, Clone)]
pub enum Content<'a> {
    Text(&'a str),
    Element(Element<'a>),
    List(ElementList<'a>),
}

impl Default for Content<'_> {
    fn default() -> Self {
        Self::Text("")
    }
}

#[derive(Debug, Clone)]
pub struct Element<'a> {
    name: &'a str,
    content: Vec<Content<'a>>,
    attrs: Vec<Attribute<'a>>,
}

impl<'a> Element<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            content: Vec::default(),
            attrs: Vec::default(),
        }
    }

    pub fn content(mut self, content: Vec<Content<'a>>) -> Self {
        self.content = content;
        self
    }

    pub fn attr(mut self, name: &'a str, value: impl Into<Cow<'a, str>>) -> Self {
        self.attrs.push(Attribute {
            name,
            value: value.into(),
        });
        self
    }

    pub fn attr_float(self, name: &'a str, value: f32) -> Self {
        let mut buf = ryu::Buffer::new();
        self.attr(name, buf.format_finite(value).to_string())
    }
}

impl<'a> Render<'a> for Element<'a> {
    fn render(&self) -> Cow<'a, str> {
        let attrs_str = self.attrs.iter().map(ToString::to_string).join(" ");
        match self.content.iter().peekable().peek() {
            None => format!("<{} {}/>", self.name, attrs_str).into(),
            Some(_) => {
                let content = self.content.iter().map(Render::render).join("");
                format!("<{0} {1}>{2}</{0}>", self.name, attrs_str, content).into()
            }
        }
    }
}

impl<'a> Render<'a> for Content<'a> {
    fn render(&self) -> Cow<'a, str> {
        match self {
            Self::Text(s) => escape_xml(s),
            Self::Element(e) => e.render(),
            Self::List(l) => l.render(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ElementList<'a> {
    content: Vec<Content<'a>>,
}

impl<'a> ElementList<'a> {
    pub fn new(content: Vec<Content<'a>>) -> Self {
        Self { content }
    }
}

impl<'a> Render<'a> for ElementList<'a> {
    fn render(&self) -> Cow<'a, str> {
        self.content.iter().map(Render::render).join("").into()
    }
}
