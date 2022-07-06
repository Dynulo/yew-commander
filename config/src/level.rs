use crate::{Color, ColorPair};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Level {
    Info,
    Danger,
    Success,
    Warning,
    Dark,
    Custom { text: ColorPair, bg: ColorPair },
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
}
