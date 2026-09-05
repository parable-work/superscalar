use crate::catalog::ScalarId;
use crate::error::{ErrorKind, ScalarError};
use crate::registry::{Registry, Scalar};
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

static RGB_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)^rgba?\s*\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*(?:,\s*([\d.]+)\s*)?\)$")
        .expect("rgb regex compiles")
});
static HSL_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?i)^hsla?\s*\(\s*([\d.]+)\s*,\s*([\d.]+)%\s*,\s*([\d.]+)%\s*(?:,\s*([\d.]+)\s*)?\)$",
    )
    .expect("hsl regex compiles")
});
static HEX_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[0-9a-fA-F]+$").expect("hex regex compiles"));
static RAW_HEX_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[0-9a-f]{6}([0-9a-f]{2})?$").expect("raw hex regex compiles"));

fn err(msg: impl Into<String>) -> ScalarError {
    ScalarError::new(ErrorKind::Parse, msg)
}

/// `clamp` guards float rounding at the 255 boundary; `as u8` then saturates (no `TryFrom<f64>`).
fn channel_u8(value: f64) -> u8 {
    value.round().clamp(0.0, 255.0) as u8
}

/// CSS Color Module Level 4 named-color set (lowercased keyword -> hex).
static NAMED_COLORS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    [
        ("aliceblue", "#f0f8ff"),
        ("antiquewhite", "#faebd7"),
        ("aqua", "#00ffff"),
        ("aquamarine", "#7fffd4"),
        ("azure", "#f0ffff"),
        ("beige", "#f5f5dc"),
        ("bisque", "#ffe4c4"),
        ("black", "#000000"),
        ("blanchedalmond", "#ffebcd"),
        ("blue", "#0000ff"),
        ("blueviolet", "#8a2be2"),
        ("brown", "#a52a2a"),
        ("burlywood", "#deb887"),
        ("cadetblue", "#5f9ea0"),
        ("chartreuse", "#7fff00"),
        ("chocolate", "#d2691e"),
        ("coral", "#ff7f50"),
        ("cornflowerblue", "#6495ed"),
        ("cornsilk", "#fff8dc"),
        ("crimson", "#dc143c"),
        ("cyan", "#00ffff"),
        ("darkblue", "#00008b"),
        ("darkcyan", "#008b8b"),
        ("darkgoldenrod", "#b8860b"),
        ("darkgray", "#a9a9a9"),
        ("darkgrey", "#a9a9a9"),
        ("darkgreen", "#006400"),
        ("darkkhaki", "#bdb76b"),
        ("darkmagenta", "#8b008b"),
        ("darkolivegreen", "#556b2f"),
        ("darkorange", "#ff8c00"),
        ("darkorchid", "#9932cc"),
        ("darkred", "#8b0000"),
        ("darksalmon", "#e9967a"),
        ("darkseagreen", "#8fbc8f"),
        ("darkslateblue", "#483d8b"),
        ("darkslategray", "#2f4f4f"),
        ("darkslategrey", "#2f4f4f"),
        ("darkturquoise", "#00ced1"),
        ("darkviolet", "#9400d3"),
        ("deeppink", "#ff1493"),
        ("deepskyblue", "#00bfff"),
        ("dimgray", "#696969"),
        ("dimgrey", "#696969"),
        ("dodgerblue", "#1e90ff"),
        ("firebrick", "#b22222"),
        ("floralwhite", "#fffaf0"),
        ("forestgreen", "#228b22"),
        ("fuchsia", "#ff00ff"),
        ("gainsboro", "#dcdcdc"),
        ("ghostwhite", "#f8f8ff"),
        ("gold", "#ffd700"),
        ("goldenrod", "#daa520"),
        ("gray", "#808080"),
        ("grey", "#808080"),
        ("green", "#008000"),
        ("greenyellow", "#adff2f"),
        ("honeydew", "#f0fff0"),
        ("hotpink", "#ff69b4"),
        ("indianred", "#cd5c5c"),
        ("indigo", "#4b0082"),
        ("ivory", "#fffff0"),
        ("khaki", "#f0e68c"),
        ("lavender", "#e6e6fa"),
        ("lavenderblush", "#fff0f5"),
        ("lawngreen", "#7cfc00"),
        ("lemonchiffon", "#fffacd"),
        ("lightblue", "#add8e6"),
        ("lightcoral", "#f08080"),
        ("lightcyan", "#e0ffff"),
        ("lightgoldenrodyellow", "#fafad2"),
        ("lightgray", "#d3d3d3"),
        ("lightgrey", "#d3d3d3"),
        ("lightgreen", "#90ee90"),
        ("lightpink", "#ffb6c1"),
        ("lightsalmon", "#ffa07a"),
        ("lightseagreen", "#20b2aa"),
        ("lightskyblue", "#87cefa"),
        ("lightslategray", "#778899"),
        ("lightslategrey", "#778899"),
        ("lightsteelblue", "#b0c4de"),
        ("lightyellow", "#ffffe0"),
        ("lime", "#00ff00"),
        ("limegreen", "#32cd32"),
        ("linen", "#faf0e6"),
        ("magenta", "#ff00ff"),
        ("maroon", "#800000"),
        ("mediumaquamarine", "#66cdaa"),
        ("mediumblue", "#0000cd"),
        ("mediumorchid", "#ba55d3"),
        ("mediumpurple", "#9370db"),
        ("mediumseagreen", "#3cb371"),
        ("mediumslateblue", "#7b68ee"),
        ("mediumspringgreen", "#00fa9a"),
        ("mediumturquoise", "#48d1cc"),
        ("mediumvioletred", "#c71585"),
        ("midnightblue", "#191970"),
        ("mintcream", "#f5fffa"),
        ("mistyrose", "#ffe4e1"),
        ("moccasin", "#ffe4b5"),
        ("navajowhite", "#ffdead"),
        ("navy", "#000080"),
        ("oldlace", "#fdf5e6"),
        ("olive", "#808000"),
        ("olivedrab", "#6b8e23"),
        ("orange", "#ffa500"),
        ("orangered", "#ff4500"),
        ("orchid", "#da70d6"),
        ("palegoldenrod", "#eee8aa"),
        ("palegreen", "#98fb98"),
        ("paleturquoise", "#afeeee"),
        ("palevioletred", "#db7093"),
        ("papayawhip", "#ffefd5"),
        ("peachpuff", "#ffdab9"),
        ("peru", "#cd853f"),
        ("pink", "#ffc0cb"),
        ("plum", "#dda0dd"),
        ("powderblue", "#b0e0e6"),
        ("purple", "#800080"),
        ("rebeccapurple", "#663399"),
        ("red", "#ff0000"),
        ("rosybrown", "#bc8f8f"),
        ("royalblue", "#4169e1"),
        ("saddlebrown", "#8b4513"),
        ("salmon", "#fa8072"),
        ("sandybrown", "#f4a460"),
        ("seagreen", "#2e8b57"),
        ("seashell", "#fff5ee"),
        ("sienna", "#a0522d"),
        ("silver", "#c0c0c0"),
        ("skyblue", "#87ceeb"),
        ("slateblue", "#6a5acd"),
        ("slategray", "#708090"),
        ("slategrey", "#708090"),
        ("snow", "#fffafa"),
        ("springgreen", "#00ff7f"),
        ("steelblue", "#4682b4"),
        ("tan", "#d2b48c"),
        ("teal", "#008080"),
        ("thistle", "#d8bfd8"),
        ("tomato", "#ff6347"),
        ("turquoise", "#40e0d0"),
        ("violet", "#ee82ee"),
        ("wheat", "#f5deb3"),
        ("white", "#ffffff"),
        ("whitesmoke", "#f5f5f5"),
        ("yellow", "#ffff00"),
        ("yellowgreen", "#9acd32"),
        ("transparent", "#00000000"),
    ]
    .into_iter()
    .collect()
});

fn named_color_hex(input: &str) -> Option<&'static str> {
    NAMED_COLORS.get(input).copied()
}

fn hex_to_rgba_hex(input: &str) -> Result<String, ScalarError> {
    let mut hex = input.trim_start_matches('#').to_string();
    if !HEX_RE.is_match(&hex) {
        return Err(err("invalid hex color format: contains non-hex characters"));
    }
    if hex.len() == 3 {
        let chars: Vec<char> = hex.chars().collect();
        hex = format!(
            "{}{}{}{}{}{}",
            chars[0], chars[0], chars[1], chars[1], chars[2], chars[2]
        );
    }
    if hex.len() == 6 {
        hex.push_str("FF");
    }
    if hex.len() != 8 {
        return Err(err("invalid hex color format"));
    }
    Ok(format!("#{}", hex.to_uppercase()))
}

fn rgb_to_rgba_hex(input: &str) -> Result<String, ScalarError> {
    let captures = RGB_RE
        .captures(input)
        .ok_or_else(|| err("invalid RGB/RGBA format"))?;
    let r = captures[1]
        .parse::<i32>()
        .map_err(|_| err("invalid RGB component"))?;
    let g = captures[2]
        .parse::<i32>()
        .map_err(|_| err("invalid RGB component"))?;
    let b = captures[3]
        .parse::<i32>()
        .map_err(|_| err("invalid RGB component"))?;
    let a = match captures.get(4) {
        Some(raw) => raw
            .as_str()
            .parse::<f64>()
            .map_err(|_| err("invalid alpha component"))?,
        None => 1.0,
    };
    if !(0..=255).contains(&r) || !(0..=255).contains(&g) || !(0..=255).contains(&b) {
        return Err(err("RGB values must be 0-255 and alpha must be 0-1"));
    }
    if !(0.0..=1.0).contains(&a) {
        return Err(err("RGB values must be 0-255 and alpha must be 0-1"));
    }
    let a_hex = channel_u8(a * 255.0);
    Ok(format!("#{r:02X}{g:02X}{b:02X}{a_hex:02X}"))
}

fn hue_to_rgb(p: f64, q: f64, mut t: f64) -> f64 {
    if t < 0.0 {
        t += 1.0;
    }
    if t > 1.0 {
        t -= 1.0;
    }
    if t < 1.0 / 6.0 {
        return p + (q - p) * 6.0 * t;
    }
    if t < 1.0 / 2.0 {
        return q;
    }
    if t < 2.0 / 3.0 {
        return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
    }
    p
}

fn hsl_to_rgba_hex(input: &str) -> Result<String, ScalarError> {
    let captures = HSL_RE
        .captures(input)
        .ok_or_else(|| err("invalid HSL/HSLA format"))?;
    let mut h = captures[1]
        .parse::<f64>()
        .map_err(|_| err("invalid hue component"))?;
    let mut s = captures[2]
        .parse::<f64>()
        .map_err(|_| err("invalid saturation component"))?;
    let mut l = captures[3]
        .parse::<f64>()
        .map_err(|_| err("invalid lightness component"))?;
    let a = match captures.get(4) {
        Some(raw) => raw
            .as_str()
            .parse::<f64>()
            .map_err(|_| err("invalid alpha component"))?,
        None => 1.0,
    };
    h /= 360.0;
    s /= 100.0;
    l /= 100.0;
    if !(0.0..=1.0).contains(&s) || !(0.0..=1.0).contains(&l) || !(0.0..=1.0).contains(&a) {
        return Err(err(
            "HSL saturation/lightness/alpha must be valid percentages",
        ));
    }
    let (r, g, b) = if s == 0.0 {
        (l, l, l)
    } else {
        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            l + s - l * s
        };
        let p = 2.0 * l - q;
        (
            hue_to_rgb(p, q, h + 1.0 / 3.0),
            hue_to_rgb(p, q, h),
            hue_to_rgb(p, q, h - 1.0 / 3.0),
        )
    };
    Ok(format!(
        "#{:02X}{:02X}{:02X}{:02X}",
        channel_u8(r * 255.0),
        channel_u8(g * 255.0),
        channel_u8(b * 255.0),
        channel_u8(a * 255.0)
    ))
}

fn normalize_color(input: &str) -> Result<String, ScalarError> {
    let trimmed = input.trim().to_lowercase();
    if trimmed.is_empty() {
        return Err(err("color value cannot be empty"));
    }
    if let Some(hex) = named_color_hex(&trimmed) {
        return hex_to_rgba_hex(hex);
    }
    if trimmed.starts_with('#') {
        return hex_to_rgba_hex(&trimmed);
    }
    if RAW_HEX_RE.is_match(&trimmed) {
        return hex_to_rgba_hex(&trimmed);
    }
    if trimmed.starts_with("rgb") {
        return rgb_to_rgba_hex(&trimmed);
    }
    if trimmed.starts_with("hsl") {
        return hsl_to_rgba_hex(&trimmed);
    }
    Err(err(format!("unsupported color format: {input}")))
}

pub struct Color;

impl Scalar for Color {
    fn id(&self) -> ScalarId {
        ScalarId::DESIGN_COLOR
    }

    fn parse(&self, _registry: &Registry, input: &str) -> Result<String, ScalarError> {
        normalize_color(input)
    }

    fn normalize(&self, _registry: &Registry, input: &str) -> Result<String, ScalarError> {
        normalize_color(input)
    }

    fn validate(&self, _registry: &Registry, input: &str) -> Result<(), ScalarError> {
        normalize_color(input).map(|_| ())
    }
}
