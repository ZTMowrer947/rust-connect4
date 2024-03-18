use std::fmt::Display;

use super::{color::Color, error::InvalidMoveError};

/** The width of the Connect 4 board. */
const POSITION_WIDTH: usize = 7;

/** The height of the Connect 4 board. */
const POSITION_HEIGHT: usize = 6;

/** Represents a Connect 4 position. */
#[derive(Clone, Copy)]
pub struct Position {
    grid: [[Option<Color>; POSITION_WIDTH]; POSITION_HEIGHT],
    col_heights: [usize; POSITION_WIDTH],
    num_moves: u8,
    player_to_move: Color,
}

impl Position {
    /** Initializes an empty board position. */
    pub fn empty() -> Self {
        Self {
            grid: [[None; POSITION_WIDTH]; POSITION_HEIGHT],
            num_moves: 0,
            col_heights: [0; POSITION_WIDTH],
            player_to_move: Color::RED,
        }
    }

    fn is_valid_col(col: usize) -> bool {
        col < POSITION_WIDTH
    }

    fn is_col_open(&self, col: usize) -> bool {
        self.col_heights[col] < POSITION_HEIGHT - 1
    }

    /** Attempts to play in the given column.
      If playing at the column is invalid, an InvalidMoveError is
      returned.
    */
    pub fn play_col(&mut self, col: usize) -> Result<(), InvalidMoveError> {
        if Self::is_valid_col(col) && self.is_col_open(col) {
            self.grid[self.col_heights[col]][col] = Some(self.player_to_move);
            self.player_to_move = self.player_to_move.opponent();
            self.col_heights[col] += 1;
            self.num_moves += 1;

            Ok(())
        } else {
            Err(InvalidMoveError::new(col, *self))
        }
    }

    /** Determine whether playing in the given column would win the game. */
    pub fn move_wins(&self, col: usize) -> bool {
        false // TODO: Implement method
    }

    /** Gets the number of moves having been played to reach this position. */
    pub fn num_moves_played(&self) -> u8 {
        self.num_moves
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let board_str = self
            .grid
            .iter()
            .rev()
            .map(|row| {
                let row_str = row
                    .iter()
                    .map(|cell| {
                        // Map to either string of color or an empty space
                        if let Some(color) = cell {
                            color.to_string()
                        } else {
                            " ".to_owned()
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("|");

                format!("|{row_str}|")
            })
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{board_str}")
    }
}

// UNIT TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_empty_creates_proper_state() {
        let pos = Position::empty();

        // Generate expected position string
        let mut expected_str = "| | | | | | | |\n".repeat(POSITION_HEIGHT);
        expected_str.pop();

        // Test num moves to play and string representation
        assert_eq!(
            pos.num_moves_played(),
            0,
            "Empty position should have no moves played yet"
        );
        assert_eq!(
            pos.to_string(),
            expected_str,
            "Position string should show blank board"
        )
    }

    #[test]
    fn position_play_col_on_empty_position_places_red_chip_on_board() {
        // Initialize empty position and board string to compare with
        let mut pos = Position::empty();
        let mut expected_str = "| | | | | | | |\n".repeat(POSITION_HEIGHT);
        expected_str.pop();

        // Define columns to test for validity and non-validity
        let valid_cols = 0..POSITION_WIDTH;
        let invalid_cols = [POSITION_WIDTH, POSITION_WIDTH + 1, usize::MAX];

        // First test invalid cols
        for col in invalid_cols {
            // Playing on out of range col should not work, nor change board state
            assert!(
                pos.play_col(col).is_err(),
                "Playing in out of range column {col} should fail"
            );

            assert_eq!(
                pos.to_string(),
                expected_str,
                "Position should still have empty board state after invalid move"
            );
            assert_eq!(
                pos.num_moves_played(),
                0,
                "Position should not update move counter after invalid move"
            );
        }

        // Next test valid cols
        for col in valid_cols {
            let mut pos_copy = pos.clone();

            // Calculate index to place expected red cell, and replace empty space with it
            let new_char_idx = (POSITION_WIDTH + 1) * 2 * (POSITION_HEIGHT - 1) + 1 + (col * 2);
            expected_str.replace_range(new_char_idx..new_char_idx + 1, "R");

            // Ensure move is accepted and board state accordingly update
            assert!(
                pos_copy.play_col(col).is_ok(),
                "Playing in the in-range column {col} should not fail"
            );
            assert_eq!(
                pos_copy.to_string(),
                expected_str,
                "Playing in column {col} should update board state with red chip"
            );

            assert_eq!(
                pos_copy.num_moves_played(),
                1,
                "Playing in column {col} should increment move count"
            );

            // Revert change of red cell in expected string
            expected_str.replace_range(new_char_idx..new_char_idx + 1, " ");
        }
    }
}
