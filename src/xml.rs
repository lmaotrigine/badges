use regex::Regex;
use std::{borrow::Cow, collections::BTreeMap};

/// A trait that indicates that a type can be rendered as XML.
pub trait Render {
    /// Render the type as an XML string.
    fn render(&self) -> String;
}

fn strip_xml_whitespace(xml: &str) -> String {
    let init = Regex::new(r">\s+").unwrap();
    let final_ = Regex::new(r"<\s+").unwrap();
    let s = init.replace_all(xml, ">");
    final_.replace_all(&s, "<").into_owned()
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
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
    attrs: BTreeMap<&'a str, Cow<'a, str>>,
}

impl<'a> Element<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            content: Vec::default(),
            attrs: BTreeMap::default(),
        }
    }

    pub fn content(mut self, content: Vec<Content<'a>>) -> Self {
        self.content = content;
        self
    }

    pub fn attr(mut self, key: &'a str, value: impl Into<Cow<'a, str>>) -> Self {
        self.attrs.insert(key, value.into());
        self
    }

    pub fn attr_float(mut self, key: &'a str, value: f32) -> Self {
        self.attrs.insert(key, Cow::from(format!("{value:.2}")));
        self
    }
}

impl<'a> Render for Element<'a> {
    fn render(&self) -> String {
        let attrs_str = self
            .attrs
            .iter()
            .map(|(k, v)| format!(" {}=\"{}\"", k, escape_xml(v)))
            .collect::<String>();
        if self.content.is_empty() {
            strip_xml_whitespace(&format!("<{}{}/>", self.name, attrs_str))
        } else {
            let content = self.content.iter().map(Render::render).collect::<String>();
            strip_xml_whitespace(&format!("<{0}{1}>{2}</{0}>", self.name, attrs_str, content))
        }
    }
}

impl Render for Content<'_> {
    fn render(&self) -> String {
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

impl Render for ElementList<'_> {
    fn render(&self) -> String {
        self.content.iter().map(Render::render).collect()
    }
}
