use thiserror::Error;

use super::position::Position;

#[derive(Error, Debug, PartialEq)]
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
