use std::{error::Error, fmt::{Debug, Display, Formatter}};

use super::position::Position;

pub struct InvalidMoveError {
    col: usize,
    position: Position,
}

impl InvalidMoveError {
    pub(crate) fn new(col: usize, pos: Position) -> Self {
        Self { col, position: pos }
    }
}

impl Debug for InvalidMoveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidMoveError: invalidCol: {}", self.col)?;
        write!(f, "\nPosition: {}", self.position)
    }
}

impl Display for InvalidMoveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cannot perform move on column {} for this position:\n{}", self.col, self.position)
    }
}

impl Error for InvalidMoveError {}