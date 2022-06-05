#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::cargo)]
#![allow(
    clippy::module_name_repetitions,
    clippy::cargo_common_metadata,
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::needless_pass_by_value,
    clippy::multiple_crate_versions,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::too_many_lines,
    clippy::similar_names,
    clippy::redundant_field_names,
    clippy::must_use_candidate
)]
#![feature(is_some_with, assert_matches)]

use std::collections::HashMap;

pub struct Board {
    pub size: u32,
    players: HashMap<u32, u32>,
}

impl Board {
    pub fn new(size: u32, number_of_players: u32) -> Self {
        Self {
            size,
            players: (0..number_of_players)
                .into_iter()
                .map(|player| (player, 0))
                .collect(),
        }
    }

    /// # Errors
    pub fn move_player(&mut self, player: u32, moves: u32) -> Result<u32, MoveError> {
        let current_position = self
            .players
            .get_mut(&player)
            .ok_or(MoveError::NonExistentPlayer)?;
        *current_position += if *current_position + moves >= self.size {
            *current_position + moves - self.size - 1
        } else {
            moves
        };
        Ok(*current_position)
    }

    /// # Errors
    pub fn move_player_to_position(
        &mut self,
        player: u32,
        position: u32,
    ) -> Result<u32, MoveError> {
        if position >= self.size {
            Err(MoveError::NonExistentPosition)
        } else {
            let current_position = self
                .players
                .get_mut(&player)
                .ok_or(MoveError::NonExistentPlayer)?;
            *current_position = position;
            Ok(*current_position)
        }
    }

    pub fn player_position(&self, player: u32) -> Option<u32> {
        self.players.get(&player).copied()
    }
}

#[derive(Debug)]
pub enum MoveError {
    NonExistentPlayer,
    NonExistentPosition,
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use crate::{Board, MoveError};

    #[test]
    fn move_without_overflow() {
        let mut board = Board {
            size: 10,
            players: [(0, 0)].into_iter().collect(),
        };

        assert_eq!(board.player_position(0).unwrap(), 0);

        let destination = board.move_player(0, 5).unwrap();

        assert_eq!(destination, 5);

        let destination = board.move_player(0, 4).unwrap();

        assert_eq!(destination, 9);
    }

    #[test]
    fn move_with_overflow() {
        let mut board = Board {
            size: 10,
            players: [(0, 0)].into_iter().collect(),
        };

        let destination = board.move_player(0, 11).unwrap();

        assert_eq!(destination, 0);
    }

    #[test]
    fn move_nonexistent_player() {
        let mut board = Board {
            size: 10,
            players: [(0, 0)].into_iter().collect(),
        };

        let destination = board.move_player(1, 5);

        assert_matches!(destination, Err(MoveError::NonExistentPlayer));
    }

    #[test]
    fn move_to_position() {
        let mut board = Board {
            size: 10,
            players: [(0, 0)].into_iter().collect(),
        };

        let mov = board.move_player_to_position(0, 4);

        assert_matches!(mov, Ok(4));
    }

    #[test]
    fn move_non_existent_player_to_position() {
        let mut board = Board {
            size: 10,
            players: [(0, 0)].into_iter().collect(),
        };

        let mov = board.move_player_to_position(1, 4);

        assert_matches!(mov, Err(MoveError::NonExistentPlayer));
    }

    #[test]
    fn move_player_to_non_existent_position() {
        let mut board = Board {
            size: 10,
            players: [(0, 0)].into_iter().collect(),
        };

        let mov = board.move_player_to_position(0, 40);

        assert_matches!(mov, Err(MoveError::NonExistentPosition));
    }
}
