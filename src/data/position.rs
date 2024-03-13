use super::color::Color;

/** The width of the Connect 4 board. */
const POSIITON_WIDTH: usize = 7;

/** The height of the Connect 4 board. */
const POSITION_HEIGHT: usize = 6;

/** Represents a Connect 4 position. */
pub(crate) struct Position {
    grid: [[Option<Color>; POSIITON_WIDTH]; POSITION_HEIGHT],
}

impl Position {
    /** Initializes an empty board position. */
    pub fn empty() -> Self {
        Position { grid: [[None; POSIITON_WIDTH]; POSITION_HEIGHT] }
    }

    /**
     * Whether this position allows playing in the given column.
     */
    pub fn can_play(&self, col: usize) -> bool {
        false // TODO: Implement method
    }

    /**
     * Play in the given column.
     */
    pub fn play_col(&self, col: usize) {
        // TODO: Implement method
    }

    /** Determine whether playing in the given column would win the game. */
    pub fn move_wins(&self, col: usize) -> bool {
        false // TODO: Implement method
    }

    /** Gets the number of moves having been played to reach this position. */
    pub fn num_moves_played() -> i32 {
        i32::MIN // TODO: Implement getter
    }
}