use std::fmt::Display;

/** Represents a colored chip on a Connect 4 board */
#[derive(Clone, Copy)]
pub(super) enum Color {
    /** A red chip */
    RED = 1,
    /** A yellow chip */
    YELLOW = 2
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // R for a red cell, Y for yellow
        let cell_str = match self {
            Self::RED => "R",
            Self::YELLOW => "Y"
        };

        write!(f, "{cell_str}")
    }
}