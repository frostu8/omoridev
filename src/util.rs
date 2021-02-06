use std::fmt::{Display, Debug, Formatter, Result as FmtResult};

use druid::Color;

/// Returns true if a color is "dark."
/// 
/// This function defines dark as any luminance below 0.5. This means that
/// colors fine tuned for light backgrounds wont work as well on this color.
/// Specifically for fonts.
pub fn is_dark(color: &Color) -> bool {
    luminance(color) < 0.5
}

/// Returns the luminance of a color, a number between `0.0` and `1.0`.
pub fn luminance(color: &Color) -> f64 {
    match *color {
        Color::Rgba32(color) => {
            let r = ((color >> 24) & 0xff) as f64 / 255.0;
            let g = ((color >> 16) & 0xff) as f64 / 255.0;
            let b = ((color >> 8) & 0xff) as f64 / 255.0;

            0.2126 * r + 0.7152 * g + 0.0722 * b
        }
    }
}

/// Wrapper for bool to display as "ON" or "OFF"
#[derive(Debug)]
pub struct BoolSwitchWrapper(bool);

impl Display for BoolSwitchWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if self.0 {
            write!(f, "ON")
        } else {
            write!(f, "OFF")
        }
    }
}

#[inline]
pub fn bool_switch(b: &bool) -> BoolSwitchWrapper {
    BoolSwitchWrapper(*b)
}