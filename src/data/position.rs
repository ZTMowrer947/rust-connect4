use std::fmt::{Debug, Display};

use super::{color::Color, error::PositionOpError};

/** The width of the Connect 4 board. */
const POSITION_WIDTH: usize = 7;

/** The height of the Connect 4 board. */
const POSITION_HEIGHT: usize = 6;

/** Represents a Connect 4 position. */
#[derive(Clone, Copy, PartialEq)]
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
            player_to_move: Color::Red,
        }
    }

    /** Places a cell of the current player at the given coordinate. */
    fn set_cell(&mut self, row: usize, col: usize) {
        self.grid[row][col] = Some(self.player_to_move);
        self.col_heights[col] += 1;
    }

    /** Attempts to find the lowest open row of the given column. */
    fn find_row(self, col: usize) -> Result<usize, PositionOpError> {
        // First verify that column is in bounds
        self.col_heights
            .get(col)
            .ok_or(PositionOpError::OutOfRangeCol(col))
            // and then ensure that the column isn't full
            .and_then(|&row| {
                self.grid
                    .get(row)
                    .map(|_| row)
                    .ok_or(PositionOpError::FullCol(col))
            })
    }

    /** Attempts to play in the given column.
      If playing at the column is invalid, an InvalidMoveError is
      returned.
    */
    pub fn play_col(&mut self, col: usize) -> Result<(), PositionOpError> {
        // Get row to play column in
        let row = self.find_row(col)?;

        // Update the board state
        self.set_cell(row, col);
        self.player_to_move = self.player_to_move.opponent();
        self.num_moves += 1;

        Ok(())
    }

    /** Determine whether playing in the given column would win the game. */
    pub fn move_wins(&self, col: usize) -> bool {
        todo!();
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

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Delegate to Display implementation
        write!(f, "{self}")
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
                pos.play_col(col)
                    .is_err_and(|err| err == PositionOpError::OutOfRangeCol(col)),
                "Playing in out of range column {col} should fail with OutOfRangeCol error"
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

    #[test]
    fn position_play_col_on_same_column_should_alternate_colors() {
        let mut pos = Position::empty();

        // Generate blank expected string board
        let mut expected_str = "| | | | | | | |\n".repeat(POSITION_HEIGHT);
        expected_str.pop();

        // Genereate alternating set of color strings
        let expected_colors = ["R", "Y", "R", "Y", "R", "Y"];
        let col = 0;

        for (col_height, color) in expected_colors.iter().enumerate() {
            // Calculate index to place expected color cell, and replace empty space with it
            let new_char_idx =
                (POSITION_WIDTH + 1) * 2 * (POSITION_HEIGHT - 1 - col_height) + 1 + (col * 2);
            expected_str.replace_range(new_char_idx..new_char_idx + 1, color);

            // Playing at this clumn should yield Ok, should increment move counter, and yield correct board state
            assert!(
                pos.play_col(col).is_ok(),
                "Playing at column {col} with height {col_height} should be valid"
            );

            assert_eq!(
                pos.num_moves_played() as usize,
                col_height + 1,
                "Number of moves should be {} after playing in column {col} that many times",
                col_height + 1
            );
        }

        assert!(
            pos.play_col(col)
                .is_err_and(|err| err == PositionOpError::FullCol(col)),
            "Playing at full column {col} should fail with FullCol error"
        );
    }

    #[test]
    fn position_move_wins_finds_winning_positions_on_all_axes() {
        // Create setup columns and column to win at
        let setup_cols = vec![
            // Two horizontal wins for red
            (vec![0, 0, 1, 1, 2, 2], 3),
            (
                vec![
                    POSITION_WIDTH - 1,
                    POSITION_WIDTH - 1,
                    POSITION_WIDTH - 2,
                    POSITION_WIDTH - 2,
                    POSITION_WIDTH - 3,
                    POSITION_WIDTH - 3,
                ],
                POSITION_WIDTH - 4,
            ),
            // Two horizontal wins for yellow
            (vec![0, 0, 1, 1, 2, 2, 4, 3, 4], 3),
            (
                vec![
                    POSITION_WIDTH - 1,
                    POSITION_WIDTH - 1,
                    POSITION_WIDTH - 2,
                    POSITION_WIDTH - 2,
                    POSITION_WIDTH - 3,
                    POSITION_WIDTH - 3,
                    POSITION_WIDTH - 5,
                    POSITION_WIDTH - 4,
                    POSITION_WIDTH - 5,
                ],
                POSITION_WIDTH - 4,
            ),
            // Two verical wins for red
            (vec![3, 4, 3, 4, 3, 4], 3),
            (vec![0, 2, 0, 1, 0, 1], 0),
            // Two vertical wins for yellow
            (vec![3, 4, 2, 4, 1, 4, 5], 4),
            (vec![0, 1, 5, 1, 3, 1, 4], 1),
            // Two diagonal wins for red
            (vec![1, 2, 2, 2, 3, 3, 3, 4, 4, 4], 4),
            (vec![0, 1, 1, 3, 2, 2, 3, 3, 3, 5], 2),
            // Two diagonal wins for yellow
            (vec![1, 3, 2, 1, 2, 2, 3, 3, 5, 3, 5], 0),
            (vec![0, 0, 0, 0, 2, 2, 2, 1, 1, 1, 2], 3),
        ];

        for (cols, winning_col) in setup_cols {
            let mut pos = Position::empty();

            for col in cols {
                assert!(
                    pos.play_col(col).is_ok(),
                    "Should be able to play at column {col} on position\n: {pos}"
                );
            }

            assert!(
                pos.move_wins(winning_col),
                "Playing at column {winning_col} should win for position\n: {pos}"
            );
        }
    }
}
