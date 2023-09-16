use crate::vendor::verdana::FONT_DATA;

const fn is_control_char(c: u32) -> bool {
    c <= 31 || c == 127
}

fn width_of_char_code(char_code: u32) -> Option<f32> {
    if is_control_char(char_code) {
        return Some(0.0);
    }
    FONT_DATA
        .binary_search_by(|(a, _, _)| a.cmp(&char_code))
        .map_or_else(
            |e| {
                let (lower, upper, width) = FONT_DATA[e - 1];
                if char_code >= lower && char_code <= upper {
                    Some(width)
                } else {
                    None
                }
            },
            |i| Some(FONT_DATA[i].2),
        )
}

pub fn measure(text: &str) -> f32 {
    let mut width = 0.0;
    for c in text.chars() {
        match width_of_char_code(c as _) {
            Some(w) => width += w,
            None => width += 10.7, // width of 'm'
        }
    }
    width
}
