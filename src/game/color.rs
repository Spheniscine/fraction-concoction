use strum_macros::{EnumCount, EnumIter};

#[derive(Clone, Copy, Debug, EnumCount, EnumIter, Hash, Eq, PartialEq)]
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Cyan
}

impl Color {
    pub fn to_html_color(&self) -> &str {
        match self {
            Color::Red => "#f00",
            Color::Green => "#0f0",
            Color::Blue => "#00f",
            Color::Yellow => "#ff0",
            Color::Cyan => "#0ff",
        }
    }
    pub fn to_tex_symbol(&self) -> &str {
        match self {
            Color::Red => "\\Delta",
            Color::Green => "\\Theta",
            Color::Blue => "\\Sigma",
            Color::Yellow => "\\Psi",
            Color::Cyan => "\\Omega",
        }
    }
    pub fn contrast_html_color(&self) -> &str {
        match self {
            Color::Red => "#fff",
            Color::Green => "#000",
            Color::Blue => "#fff",
            Color::Yellow => "#000",
            Color::Cyan => "#000",
        }
    }
}