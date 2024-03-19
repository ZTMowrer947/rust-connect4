use std::{error::Error, fmt::{Debug, Display, Formatter}};

use thiserror::Error;

use super::position::Position;

#[derive(Error, Debug)]
pub enum PositionOpError {
    #[error("Playing at column {} would be out of bounds", .0 + 1)]
    OutOfRangeCol(usize),

    #[error("Column {} is full", .0 + 1)]
    FullCol(usize),

    #[error("Invalid sequence '{seq}': {source}, detected at index {bad_idx}")]
    InvalidSequence {
        #[source]
        source: Box<PositionOpError>,
        seq: String,
        bad_idx: usize
    },

    #[error("Playing at column {} would prematurely solve position: \n{pos}", .col + 1)]
    PrematureSolve {
        col: usize,
        pos: Position,
    },
}

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