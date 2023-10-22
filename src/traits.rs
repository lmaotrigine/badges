use std::{borrow::Cow, fmt::Write};

/// A trait that indicates that a type can be rendered as XML.
pub trait Render<'a> {
    /// Render the type as an XML string.
    fn render(&self) -> Cow<'a, str>;
}

pub trait IteratorExt: Iterator {
    fn join(&mut self, sep: &str) -> String
    where
        Self::Item: std::fmt::Display,
    {
        self.next().map_or_else(String::new, |elem| {
            let (cap, _) = self.size_hint();
            let mut res = String::with_capacity(cap * sep.len());
            write!(&mut res, "{elem}").expect(":derp:");
            self.for_each(|elem| {
                res.push_str(sep);
                write!(&mut res, "{elem}").expect(":derp:");
            });
            res
        })
    }
}

impl<T: Iterator> IteratorExt for T {}
