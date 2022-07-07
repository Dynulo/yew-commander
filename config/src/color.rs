use yew::prelude::*;

macro_rules! color {
    ($($name:ident),*) => {
        paste::item! {
            #[derive(Clone, Copy, Debug, PartialEq)]
            pub enum Color {
                $(
                    [<$name 50>],
                    [<$name 100>],
                    [<$name 200>],
                    [<$name 300>],
                    [<$name 400>],
                    [<$name 500>],
                    [<$name 600>],
                    [<$name 700>],
                    [<$name 800>],
                    [<$name 900>],
                )*
                White,
            }
        }
    };
}

macro_rules! color_hex {
    ($($name:ident => {
        $a50: expr,
        $a100: expr,
        $a200: expr,
        $a300: expr,
        $a400: expr,
        $a500: expr,
        $a600: expr,
        $a700: expr,
        $a800: expr,
        $a900: expr,
    }),*) => {
        paste::item! {
            pub fn as_hex(&self) -> String {
                match self {
                    $(
                        Color::[<$name 50>] => $a50,
                        Color::[<$name 100>] => $a100,
                        Color::[<$name 200>] => $a200,
                        Color::[<$name 300>] => $a300,
                        Color::[<$name 400>] => $a400,
                        Color::[<$name 500>] => $a500,
                        Color::[<$name 600>] => $a600,
                        Color::[<$name 700>] => $a700,
                        Color::[<$name 800>] => $a800,
                        Color::[<$name 900>] => $a900,
                    )*
                    Color::White => "#FFFFFF",
                }.to_string()
            }

            pub fn as_class(&self) -> String {
                match self {
                    $(
                        Color::[<$name 50>] => format!("{}-50", stringify!($name).to_lowercase()),
                        Color::[<$name 100>] => format!("{}-100", stringify!($name).to_lowercase()),
                        Color::[<$name 200>] => format!("{}-200", stringify!($name).to_lowercase()),
                        Color::[<$name 300>] => format!("{}-300", stringify!($name).to_lowercase()),
                        Color::[<$name 400>] => format!("{}-400", stringify!($name).to_lowercase()),
                        Color::[<$name 500>] => format!("{}-500", stringify!($name).to_lowercase()),
                        Color::[<$name 600>] => format!("{}-600", stringify!($name).to_lowercase()),
                        Color::[<$name 700>] => format!("{}-700", stringify!($name).to_lowercase()),
                        Color::[<$name 800>] => format!("{}-800", stringify!($name).to_lowercase()),
                        Color::[<$name 900>] => format!("{}-900", stringify!($name).to_lowercase()),
                    )*
                    Color::White => "white".to_string(),
                }
            }

            pub fn every_color() -> String {
                let mut colors = String::new();
                $(
                    let name = stringify!($name).to_lowercase();
                    for p in &["text", "bg", "border", "dark:text", "dark:bg"] {
                        for i in &[50, 100, 200, 300, 400, 500, 600, 700, 800, 900] {
                            colors.push_str(p);
                            colors.push_str("-");
                            colors.push_str(&name);
                            colors.push_str("-");
                            colors.push_str(i.to_string().as_str());
                            colors.push_str(" ");
                        }
                    }
                )*
                colors
            }
        }
    };
}

color!(
    Slate, Gray, Zinc, Neutral, Stone, Red, Orange, Amber, Yellow, Lime, Green, Emerald, Teal,
    Cyan, Sky, Blue, Indigo, Violet, Purple, Fuchsia, Pink, Rose
);

impl Color {
    pub fn as_text_class(&self) -> String {
        format!("text-{}", self.as_class())
    }

    pub fn as_bg_class(&self) -> String {
        format!("bg-{}", self.as_class())
    }

    pub fn as_border_class(&self) -> String {
        format!("border-{}", self.as_class())
    }

    color_hex! {
        Slate => {
            "#F8FAFC",
            "#F1F5F9",
            "#E2E8F0",
            "#CBD5E1",
            "#94A3B8",
            "#64748B",
            "#475569",
            "#334155",
            "#1E293B",
            "#0F172A",
        },
        Gray => {
            "#F9FAFB",
            "#F3F4F6",
            "#E5E7EB",
            "#D1D5DB",
            "#9CA3AF",
            "#6B7280",
            "#4B5563",
            "#374151",
            "#1F2937",
            "#111827",
        },
        Zinc => {
            "#FAFAFA",
            "#F4F4F5",
            "#E4E4E7",
            "#D4D4D8",
            "#A1A1AA",
            "#71717A",
            "#52525B",
            "#3F3F46",
            "#27272A",
            "#18181B",
        },
        Neutral => {
            "#FAFAFA",
            "#F5F5F5",
            "#E5E5E5",
            "#D4D4D4",
            "#A3A3A3",
            "#737373",
            "#525252",
            "#404040",
            "#262626",
            "#171717",
        },
        Stone => {
            "#FAFAF9",
            "#F5F5F4",
            "#E7E5E4",
            "#D6D3D1",
            "#A8A29E",
            "#78716C",
            "#57534E",
            "#44403C",
            "#292524",
            "#1C1917",
        },
        Red => {
            "#FEF2F2",
            "#FEE2E2",
            "#FECACA",
            "#FCA5A5",
            "#F87171",
            "#EF4444",
            "#DC2626",
            "#B91C1C",
            "#991B1B",
            "#7F1D1D",
        },
        Orange => {
            "#FFF7ED",
            "#FFEDD5",
            "#FED7AA",
            "#FDBA74",
            "#FB923C",
            "#F97316",
            "#EA580C",
            "#C2410C",
            "#9A3412",
            "#7C2D12",
        },
        Amber => {
            "#FFFBEB",
            "#FEF3C7",
            "#FDE68A",
            "#FCD34D",
            "#FBBF24",
            "#F59E0B",
            "#D97706",
            "#B45309",
            "#92400E",
            "#78350F",
        },
        Yellow => {
            "#FEFCE8",
            "#FEF9C3",
            "#FEF08A",
            "#FDE047",
            "#FACC15",
            "#EAB308",
            "#CA8A04",
            "#A16207",
            "#854D0E",
            "#713F12",
        },
        Lime => {
            "#F7FEE7",
            "#ECFCCB",
            "#D9F99D",
            "#BEF264",
            "#A3E635",
            "#84CC16",
            "#65A30D",
            "#4D7C0F",
            "#3F6212",
            "#365314",
        },
        Green => {
            "#F0FDF4",
            "#DCFCE7",
            "#BBF7D0",
            "#86EFAC",
            "#4ADE80",
            "#22C55E",
            "#16A34A",
            "#15803D",
            "#166534",
            "#14532D",
        },
        Emerald => {
            "#ECFDF5",
            "#D1FAE5",
            "#A7F3D0",
            "#6EE7B7",
            "#34D399",
            "#10B981",
            "#059669",
            "#047857",
            "#065F46",
            "#064E3B",
        },
        Teal => {
            "#F0FDFA",
            "#CCFBF1",
            "#99F6E4",
            "#5EEAD4",
            "#2DD4BF",
            "#14B8A6",
            "#0D9488",
            "#0F766E",
            "#115E59",
            "#134E4A",
        },
        Cyan => {
            "#ECFEFF",
            "#CFFAFE",
            "#A5F3FC",
            "#67E8F9",
            "#22D3EE",
            "#06B6D4",
            "#0891B2",
            "#0E7490",
            "#155E75",
            "#164E63",
        },
        Sky => {
            "#F0F9FF",
            "#E0F2FE",
            "#BAE6FD",
            "#7DD3FC",
            "#38BDF8",
            "#0EA5E9",
            "#0284C7",
            "#0369A1",
            "#075985",
            "#0C4A6E",
        },
        Blue => {
            "#EFF6FF",
            "#DBEAFE",
            "#BFDBFE",
            "#93C5FD",
            "#60A5FA",
            "#3B82F6",
            "#2563EB",
            "#1D4ED8",
            "#1E40AF",
            "#1E3A8A",
        },
        Indigo => {
            "#EEF2FF",
            "#E0E7FF",
            "#C7D2FE",
            "#A5B4FC",
            "#818CF8",
            "#6366F1",
            "#4F46E5",
            "#4338CA",
            "#3730A3",
            "#312E81",
        },
        Violet => {
            "#F5F3FF",
            "#EDE9FE",
            "#DDD6FE",
            "#C4B5FD",
            "#A78BFA",
            "#8B5CF6",
            "#7C3AED",
            "#6D28D9",
            "#5B21B6",
            "#4C1D95",
        },
        Purple => {
            "#FAF5FF",
            "#F3E8FF",
            "#E9D5FF",
            "#D8B4FE",
            "#C084FC",
            "#A855F7",
            "#9333EA",
            "#7E22CE",
            "#6B21A8",
            "#581C87",
        },
        Fuchsia => {
            "#FDF4FF",
            "#FAE8FF",
            "#F5D0FE",
            "#F0ABFC",
            "#E879F9",
            "#D946EF",
            "#C026D3",
            "#A21CAF",
            "#86198F",
            "#701A75",
        },
        Pink => {
            "#FDF2F8",
            "#FCE7F3",
            "#FBCFE8",
            "#F9A8D4",
            "#F472B6",
            "#EC4899",
            "#DB2777",
            "#BE185D",
            "#9D174D",
            "#831843",
        },
        Rose => {
            "#FFF1F2",
            "#FFE4E6",
            "#FECDD3",
            "#FDA4AF",
            "#FB7185",
            "#F43F5E",
            "#E11D48",
            "#BE123C",
            "#9F1239",
            "#881337",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorPair {
    light: Color,
    dark: Color,
}

impl ColorPair {
    pub fn new(light: Color, dark: Color) -> ColorPair {
        ColorPair { light, dark }
    }

    pub fn as_text_classes(&self) -> Classes {
        classes!(
            self.light.as_text_class(),
            format!("dark:{}", self.dark.as_text_class()),
        )
    }

    pub fn as_bg_classes(&self) -> Classes {
        classes!(
            self.light.as_bg_class(),
            format!("dark:{}", self.dark.as_bg_class()),
        )
    }

    pub fn as_border_classes(&self) -> Classes {
        classes!(
            self.light.as_border_class(),
            format!("dark:{}", self.dark.as_border_class()),
        )
    }
}
