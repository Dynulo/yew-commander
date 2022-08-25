use crate::{Color, ColorPair};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Level {
    #[default]
    Info,
    Danger,
    Success,
    Warning,
    Dark,
    Custom {
        text: ColorPair,
        bg: ColorPair,
        border: Color,
    },
}

impl Level {
    pub const fn text(&self) -> ColorPair {
        match *self {
            Self::Info => ColorPair::new(Color::Blue700, Color::Blue800),
            Self::Danger => ColorPair::new(Color::Red700, Color::Red800),
            Self::Success => ColorPair::new(Color::Green700, Color::Green800),
            Self::Warning => ColorPair::new(Color::Yellow700, Color::Yellow800),
            Self::Dark => ColorPair::new(Color::Gray700, Color::Gray800),
            Self::Custom { text, .. } => text,
        }
    }

    pub const fn bg(&self) -> ColorPair {
        match *self {
            Self::Info => ColorPair::new(Color::Blue100, Color::Blue200),
            Self::Danger => ColorPair::new(Color::Red100, Color::Red200),
            Self::Success => ColorPair::new(Color::Green100, Color::Green200),
            Self::Warning => ColorPair::new(Color::Yellow100, Color::Yellow200),
            Self::Dark => ColorPair::new(Color::Gray100, Color::Gray200),
            Self::Custom { bg, .. } => bg,
        }
    }

    pub const fn border(&self) -> Color {
        match *self {
            Self::Info => Color::Blue500,
            Self::Danger => Color::Red500,
            Self::Success => Color::Green500,
            Self::Warning => Color::Yellow500,
            Self::Dark => Color::Gray500,
            Self::Custom { border, .. } => border,
        }
    }
}
