use std::fmt::Display;

/** Represents a colored chip on a Connect 4 board */
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum Color {
    /** A red chip */
    Red = 1,
    /** A yellow chip */
    Yellow = 2
}

impl Color {
    /** Returns the color of the opponent player. */
    pub fn opponent(&self) -> Self {
        match self {
            Self::Red => Self::Yellow,
            Self::Yellow => Self::Red
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // R for a red cell, Y for yellow
        let cell_str = match self {
            Self::Red => "R",
            Self::Yellow => "Y"
        };

        write!(f, "{cell_str}")
    }
}