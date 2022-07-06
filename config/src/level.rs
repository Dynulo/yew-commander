use crate::{Color, ColorPair};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Level {
    Info,
    Danger,
    Success,
    Warning,
    Dark,
    Custom { text: ColorPair, bg: ColorPair, border: Color },
}

impl Level {
    pub fn text(&self) -> ColorPair {
        match *self {
            Level::Info => ColorPair::new(Color::Blue700, Color::Blue800),
            Level::Danger => ColorPair::new(Color::Red700, Color::Red800),
            Level::Success => ColorPair::new(Color::Green700, Color::Green800),
            Level::Warning => ColorPair::new(Color::Yellow700, Color::Yellow800),
            Level::Dark => ColorPair::new(Color::Gray700, Color::Gray800),
            Level::Custom { text, .. } => text,
        }
    }

    pub fn bg(&self) -> ColorPair {
        match *self {
            Level::Info => ColorPair::new(Color::Blue100, Color::Blue200),
            Level::Danger => ColorPair::new(Color::Red100, Color::Red200),
            Level::Success => ColorPair::new(Color::Green100, Color::Green200),
            Level::Warning => ColorPair::new(Color::Yellow100, Color::Yellow200),
            Level::Dark => ColorPair::new(Color::Gray100, Color::Gray200),
            Level::Custom { bg, .. } => bg,
        }
    }

    pub fn border(&self) -> Color {
        match *self {
            Level::Info => Color::Blue500,
            Level::Danger => Color::Red500,
            Level::Success => Color::Green500,
            Level::Warning => Color::Yellow500,
            Level::Dark => Color::Gray500,
            Level::Custom { border, .. } => border,
        }
    }
}
