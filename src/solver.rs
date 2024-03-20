use crate::data::position::{Position, POSITION_HEIGHT, POSITION_WIDTH};

/** A struct for solving Connect 4 positions. */
pub struct Solver;

const SCORE_MAX: i32 = (POSITION_HEIGHT * POSITION_WIDTH) as i32;
const SCORE_MIN: i32 = -SCORE_MAX;

impl Solver {
    // TODO: Provide more suitable error type
    fn score_position(pos: &mut Position) -> Result<i32, ()> {
        todo!();
    }

    pub fn solve_sequence(cols: impl Iterator<Item = usize>) -> Result<i32, ()> {
        todo!();
    }
}
