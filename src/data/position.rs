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
    player_to_move: Color
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

    /** Attempts to play in the given column.
        If playing at the column is invalid, an InvalidMoveError is
        returned.
      */
    pub fn play_col(&mut self, col: usize) -> Result<(), InvalidMoveError> {
        if Self::is_valid_col(col) {
            // TODO: Actually make the move
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
    pub fn num_moves_played() -> i32 {
        i32::MIN // TODO: Implement getter
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let board_str = self.grid.iter().rev().map(|row| {
            let row_str = row.iter().map(|cell| {
                if let Some(color) = cell {
                    color.to_string()
                } else {
                    " ".to_owned()
                }
            }).collect::<Vec<String>>()
            .join("|");

            "|".to_owned() + &row_str + "|"
        }).collect::<Vec<String>>().join("\n");

        write!(f, "{board_str}")
    }
}