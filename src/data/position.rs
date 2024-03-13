/** The width of the Connect 4 board. */
const POSIITON_WIDTH: usize = 7;

/** The height of the Connect 4 board. */
const POSITION_HEIGHT: usize = 6;

pub(crate) struct Position {    
}

impl Position {
    pub fn can_play(&self, col: usize) -> bool {
        false // TODO: Implement method
    }

    pub fn play_col(&self, col: usize) {
        // TODO: Implement method
    }

    pub fn move_wins(&self, col: usize) -> bool {
        false // TODO: Implement method
    }

    pub fn num_moves_played() -> i32 {
        i32::MIN // TODO: Implement getter
    }
}