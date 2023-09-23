use crate::{
    colour::Colour,
    font::measure,
    xml::{Content, Element, ElementList, Render},
    BRIGHTNESS_THRESHOLD, FONT_FAMILY, FONT_SCALE_DOWN_VALUE, FONT_SCALE_UP_FACTOR, HEIGHT,
    LOGO_HEIGHT, SHADOW, VERTICAL_MARGIN,
};

const fn colours_for_background(colour: Colour) -> (Colour, Colour) {
    if colour.brightness() <= BRIGHTNESS_THRESHOLD {
        (
            Colour::from_colour_code(0xff_ff_ff),
            Colour::from_colour_code(0x01_01_01),
        )
    } else {
        (
            Colour::from_colour_code(0x33_33_33),
            Colour::from_colour_code(0xcc_cc_cc),
        )
    }
}

fn round_up_to_odd(val: f32) -> f32 {
    if val.rem_euclid(2.0) == 0.0 {
        val + 1.0
    } else {
        val
    }
}

fn preferred_width(text: &str) -> f32 {
    if text.is_empty() {
        return 0.0;
    }
    round_up_to_odd(measure(text))
}

/// A Badge struct represents a badge that can be rendered to SVG.
///
/// # Example
///
/// ```rust
/// use badges::{Badge, Colour, Render};
/// let badge = Badge::builder().message("Hello").build();
/// println!("{}", badge.render());
/// ```
#[derive(Debug, Clone)]
pub struct Badge<'a> {
    label_colour: Colour,
    colour: Colour,
    label: Option<&'a str>,
    message: &'a str,
    logo: Option<&'a str>,
    logo_width: Option<f32>,
}

/// A Builder struct represents a builder for a [`Badge`].
#[derive(Debug, Clone, Default)]
pub struct Builder<'a> {
    label_colour: Option<Colour>,
    colour: Option<Colour>,
    label: Option<&'a str>,
    message: &'a str,
    logo: Option<&'a str>,
    logo_width: Option<f32>,
}

impl<'a> Builder<'a> {
    /// Set the message for the badge.
    #[must_use]
    pub const fn message(mut self, message: &'a str) -> Self {
        self.message = message;
        self
    }

    /// Set the label for the badge.
    #[must_use]
    pub const fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    /// Set the colour for the badge.
    #[must_use]
    pub const fn colour(mut self, colour: Colour) -> Self {
        self.colour = Some(colour);
        self
    }

    /// Set the label colour for the badge.
    #[must_use]
    pub const fn label_colour(mut self, label_colour: Colour) -> Self {
        self.label_colour = Some(label_colour);
        self
    }

    /// Set the logo for the badge.
    #[must_use]
    pub const fn logo(mut self, logo: &'a str) -> Self {
        self.logo = Some(logo);
        self
    }

    /// Set the logo width for the badge.
    #[must_use]
    pub const fn logo_width(mut self, logo_width: f32) -> Self {
        self.logo_width = Some(logo_width);
        self
    }

    /// Build the [`Badge`].
    #[must_use]
    pub fn build(self) -> Badge<'a> {
        Badge::new(
            self.label,
            self.message,
            self.logo,
            self.logo_width,
            self.colour,
            self.label_colour,
        )
    }
}

impl<'a> Badge<'a> {
    const HORIZ_PADDING: f32 = 5.0;

    #[must_use]
    /// Create a new [`Builder`] for a [`Badge`].
    ///
    /// This method returns a [`Builder`] with all fields set to their default values.
    /// After customizing the badge with the relevant methods, call [`Builder::build`] to
    /// create the [`Badge`].
    pub fn builder() -> Builder<'a> {
        Builder::default()
    }

    fn new(
        label: Option<&'a str>,
        message: &'a str,
        logo: Option<&'a str>,
        logo_width: Option<f32>,
        colour: Option<Colour>,
        label_colour: Option<Colour>,
    ) -> Self {
        let colour = colour.unwrap_or(Colour::from_colour_code(0x44_cc_11));
        let label_colour = label_colour.unwrap_or(Colour::from_colour_code(0x55_55_55));
        Self {
            label_colour,
            colour,
            label,
            message,
            logo,
            logo_width,
        }
    }

    const fn label_colour(&self) -> Colour {
        if self.has_label() || self.has_logo() {
            self.label_colour
        } else {
            self.colour
        }
    }
    const fn has_logo(&self) -> bool {
        matches!(self.logo, Some(l) if !l.is_empty())
    }

    const fn has_label(&self) -> bool {
        matches!(self.label, Some(l) if !l.is_empty())
    }

    const fn logo_width(&self) -> f32 {
        match self.logo_width {
            Some(w) => w,
            None => {
                if self.has_logo() {
                    14.0
                } else {
                    0.0
                }
            }
        }
    }

    const fn logo_padding(&self) -> f32 {
        if self.has_label() && self.has_logo() {
            3.0
        } else {
            0.0
        }
    }

    fn accessible_text(&self) -> String {
        self.label.map_or_else(
            || self.message.to_string(),
            |label| format!("{label}: {message}", message = self.message),
        )
    }

    fn label_margin(&self) -> f32 {
        self.logo_width() + self.logo_padding() + 1.0
    }

    fn label_width(&self) -> f32 {
        preferred_width(self.label.unwrap_or_default())
    }

    fn left_width(&self) -> f32 {
        if self.has_label() {
            2.0f32.mul_add(Self::HORIZ_PADDING, self.label_width())
                + self.logo_padding()
                + self.logo_width()
        } else {
            0.0
        }
    }

    #[allow(clippy::cast_precision_loss)]
    fn message_margin(&self) -> f32 {
        let mut mar = self.left_width() - self.message.len().min(1) as f32;
        if !self.has_label() {
            if self.has_logo() {
                mar += self.logo_padding() + self.logo_width() + Self::HORIZ_PADDING;
            } else {
                mar += 1.0;
            }
        }
        mar
    }

    fn message_width(&self) -> f32 {
        preferred_width(self.message)
    }

    fn right_width(&self) -> f32 {
        let mut width = 2.0f32.mul_add(Self::HORIZ_PADDING, self.message_width());
        if self.has_logo() && !self.has_label() {
            width += self.logo_padding() + self.logo_width() + Self::HORIZ_PADDING - 1.0;
        }
        width
    }

    fn width(&self) -> f32 {
        self.left_width() + self.right_width()
    }

    fn logo_element(&self) -> Content<'a> {
        self.logo.map_or_else(Content::default, |logo| {
            Content::Element(
                Element::new("image")
                    .attr_float("x", Self::HORIZ_PADDING)
                    .attr_float("y", 0.5 * (HEIGHT - LOGO_HEIGHT))
                    .attr_float("width", self.logo_width())
                    .attr_float("height", LOGO_HEIGHT)
                    .attr("xlink:href", logo),
            )
        })
    }

    fn _text_element(content: &'a str, colour: Colour, width: f32, margin: f32) -> Content<'a> {
        if content.is_empty() {
            return Content::default();
        }
        let (text_colour, shadow_colour) = colours_for_background(colour);
        let x = FONT_SCALE_UP_FACTOR * (0.5f32.mul_add(width, margin) + Self::HORIZ_PADDING);
        let text = Element::new("text")
            .content(vec![Content::Text(content)])
            .attr_float("x", x)
            .attr_float("y", 140.0 + VERTICAL_MARGIN)
            .attr("transform", FONT_SCALE_DOWN_VALUE)
            .attr("fill", text_colour.to_string())
            .attr_float("textLength", FONT_SCALE_UP_FACTOR * width);
        let shadow_text = Element::new("text")
            .content(vec![Content::Text(content)])
            .attr("aria-hidden", "true")
            .attr_float("x", x)
            .attr_float("y", 150.0 + VERTICAL_MARGIN)
            .attr("transform", FONT_SCALE_DOWN_VALUE)
            .attr("fill", shadow_colour.to_string())
            .attr("fill-opacity", ".3")
            .attr_float("textLength", FONT_SCALE_UP_FACTOR * width);
        let shadow = if SHADOW {
            Content::Element(shadow_text)
        } else {
            Content::default()
        };
        Content::List(ElementList::new(vec![shadow, Content::Element(text)]))
    }

    fn label_element(&self) -> Content<'a> {
        Self::_text_element(
            self.label.unwrap_or_default(),
            self.label_colour,
            self.label_width(),
            self.label_margin(),
        )
    }

    fn message_element(&self) -> Content<'a> {
        Self::_text_element(
            self.message,
            self.colour,
            self.message_width(),
            self.message_margin(),
        )
    }

    fn clip_path(&self, rx: f32) -> Content<'a> {
        Content::Element(
            Element::new("clipPath")
                .content(vec![Content::Element(
                    Element::new("rect")
                        .attr_float("width", self.width())
                        .attr_float("height", HEIGHT)
                        .attr_float("rx", rx)
                        .attr("fill", "#fff"),
                )])
                .attr("id", "r"),
        )
    }

    fn background_group_element(&self, gradient: bool) -> Element {
        let left_rect = Element::new("rect")
            .attr_float("width", self.left_width())
            .attr_float("height", HEIGHT)
            .attr("fill", self.label_colour().to_string());
        let right_rect = Element::new("rect")
            .attr_float("x", self.left_width())
            .attr_float("width", self.right_width())
            .attr_float("height", HEIGHT)
            .attr("fill", self.colour.to_string());
        let mut content = vec![Content::Element(left_rect), Content::Element(right_rect)];
        if gradient {
            let grad = Element::new("rect")
                .attr_float("width", self.width())
                .attr_float("height", HEIGHT)
                .attr("fill", "url(#s)");
            content.push(Content::Element(grad));
        }
        Element::new("g").content(content)
    }

    fn foreground_group_element(&self) -> Content<'a> {
        Content::Element(
            Element::new("g")
                .content(vec![
                    self.logo_element(),
                    self.label_element(),
                    self.message_element(),
                ])
                .attr("fill", "#fff")
                .attr("text-anchor", "middle")
                .attr("font-family", FONT_FAMILY)
                .attr("text-rendering", "geometricPrecision")
                .attr("font-size", "110"),
        )
    }
}

impl<'a> Render for Badge<'a> {
    fn render(&self) -> String {
        let gradient = Element::new("linearGradient")
            .content(vec![
                Content::Element(
                    Element::new("stop")
                        .attr("offset", "0")
                        .attr("stop-color", "#bbb")
                        .attr("stop-opacity", ".1"),
                ),
                Content::Element(
                    Element::new("stop")
                        .attr("offset", "1")
                        .attr("stop-opacity", ".1"),
                ),
            ])
            .attr("id", "s")
            .attr("x2", "0")
            .attr("y2", "100%");
        let clip_path = self.clip_path(3.0);
        let background_group = self
            .background_group_element(true)
            .attr("clip-path", "url(#r)");
        let body = Content::List(ElementList::new(vec![
            Content::Element(gradient),
            clip_path,
            Content::Element(background_group),
            self.foreground_group_element(),
        ]));
        let width = self.left_width() + self.right_width();
        let a11y_text = self.accessible_text();
        let title =
            Content::Element(Element::new("title").content(vec![Content::Text(&a11y_text)]));
        let svg = Element::new("svg")
            .content(vec![title, body])
            .attr("xmlns", "http://www.w3.org/2000/svg")
            .attr("xmlns:xlink", "http://www.w3.org/1999/xlink")
            .attr_float("width", width)
            .attr_float("height", HEIGHT)
            .attr("role", "img")
            .attr("aria-label", self.accessible_text());
        svg.render()
    }
}
