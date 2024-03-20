use crate::data::position::{Position, POSITION_HEIGHT, POSITION_WIDTH};

/** A struct for solving Connect 4 positions. */
pub struct Solver;

const SCORE_MAX: i32 = (POSITION_HEIGHT * POSITION_WIDTH) as i32;
const SCORE_MIN: i32 = -SCORE_MAX;

impl Solver {
    // TODO: Provide more suitable error type
    fn score_position(&self, pos: &mut Position) -> Result<i32, ()> {
        todo!();
    }

    pub fn solve_sequence(&self, cols: impl Iterator<Item = usize>) -> Result<i32, ()> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solver_solves_endpoint_sequence() {
        let solver = Solver;

        let expected_score = -1;
        let cols = [
            1usize, 1, 4, 1, 4, 6, 5, 1, 4, 2, 3, 5, 1, 1, 3, 3, 0, 0, 0, 4, 5, 2, 2, 5, 4, 2, 3,
            2, 5, 6, 0, 2, 4, 0, 3, 3, 0,
        ]
        .into_iter();

        assert_eq!(
            solver.solve_sequence(cols),
            Ok(expected_score),
            "Sequence should yield score of {expected_score}"
        );
    }
}
