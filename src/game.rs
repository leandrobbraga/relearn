use std::fmt::Display;
use std::hash::Hash;

use serde::{Deserialize, Serialize};

/// Helper macro to make the board easier to see for humans, it enable us to define a board state
/// like this:
///
/// # Example
///
/// ```
///     fields![
///         X O X
///         - X -
///         O O X
///     ]
/// ```
macro_rules! fields {
    (O) => {Some(Player::O)};
    (X) => {Some(Player::X)};
    (-) => {None};
    (_) => {_};
    ($($s:tt)+) => {
        [$(fields!($s)),+]
    };
}

/// A basic game implementation (Tic-Tac-Toe).
pub struct Game;

#[derive(Clone, Debug, Eq, Serialize, Deserialize)]
pub struct State {
    fields: [Option<Player>; 9],
    available_fields: Vec<u8>,
}

impl State {
    /// Encode the state in a single u16 number that can be used either for simpler hashing or for
    /// array indexing.
    pub fn encode_state(&self) -> u16 {
        // The state is an array representing the board fields, containing 9 elements each with 3
        // possibilities, Empty, Player:X and Player:O.
        // To make hashing faster we are transforming this array in an unique integer.
        let mut acc: u16 = 0;

        for (idx, field) in self.fields.iter().enumerate() {
            acc += match field {
                Some(player) => match player {
                    Player::O => 2,
                    Player::X => 1,
                },
                None => 0,
            } * u16::pow(3, idx as u32)
        }

        acc
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    Finished(Option<Player>),
    OnGoing,
}

#[derive(Debug)]
pub enum MoveError {
    NonEmptyField,
    OutOfBound,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Player {
    X,
    O,
}

impl Game {
    pub(crate) fn play(
        player_1: &dyn crate::Player,
        player_2: &dyn crate::Player,
    ) -> Option<Player> {
        let mut current_player = Player::X;
        let mut board = State::new();

        loop {
            let next_player = current_player.next_player();
            let player = std::mem::replace(&mut current_player, next_player);

            let action = match player {
                Player::X => player_1.play(&board, player),
                Player::O => player_2.play(&board, player),
            };

            if Game::act(player, action, &mut board).is_err() {
                // The same player tries again
                current_player = current_player.next_player();
                continue;
            };

            if let Status::Finished(winner) = Game::status(&board) {
                break winner;
            }
        }
    }

    pub(crate) fn status(state: &State) -> Status {
        let winner = Game::winner(state);

        if winner.is_some() {
            Status::Finished(winner)
        } else if state.available_fields.is_empty() {
            Status::Finished(None)
        } else {
            Status::OnGoing
        }
    }

    pub(crate) fn available_moves(state: &State) -> &Vec<u8> {
        &state.available_fields
    }

    /// Act in the state, mutating it.
    ///
    /// For now we'll keep this method as fallible for debugging purpose, we might implement a
    /// `unchecked_act` in the future for optimization purpose.
    pub(crate) fn act(player: Player, position: u8, state: &mut State) -> Result<(), MoveError> {
        state.act(player, position)
    }

    fn winner(state: &State) -> Option<Player> {
        // it's not possible to have a winner with that few plays
        if state.available_fields.len() > 4 {
            return None;
        }

        match state.fields {
            fields![
                X X X
                _ _ _
                _ _ _
            ]
            | fields![
                _ _ _
                X X X
                _ _ _
            ]
            | fields![
                _ _ _
                _ _ _
                X X X
            ]
            | fields![
                X _ _
                X _ _
                X _ _
            ]
            | fields![
                _ X _
                _ X _
                _ X _
            ]
            | fields![
                _ _ X
                _ _ X
                _ _ X
            ]
            | fields![
                X _ _
                _ X _
                _ _ X
            ]
            | fields![
                _ _ X
                _ X _
                X _ _
            ] => Some(Player::X),
            fields![
                O O O
                _ _ _
                _ _ _
            ]
            | fields![
                _ _ _
                O O O
                _ _ _
            ]
            | fields![
                _ _ _
                _ _ _
                O O O
            ]
            | fields![
                O _ _
                O _ _
                O _ _
            ]
            | fields![
                _ O _
                _ O _
                _ O _
            ]
            | fields![
                _ _ O
                _ _ O
                _ _ O
            ]
            | fields![
                O _ _
                _ O _
                _ _ O
            ]
            | fields![
                _ _ O
                _ O _
                O _ _
            ] => Some(Player::O),
            _ => None,
        }
    }
}

impl State {
    pub(crate) fn new() -> Self {
        State {
            fields: Default::default(),
            available_fields: vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
        }
    }

    #[cfg(test)]
    fn from_array(fields: [Option<Player>; 9]) -> Self {
        let available_fields = fields
            .iter()
            .enumerate()
            .filter_map(|(idx, field)| {
                if field.is_none() {
                    Some(idx as u8)
                } else {
                    None
                }
            })
            .collect();

        Self {
            fields,
            available_fields,
        }
    }

    fn act(&mut self, player: Player, position: u8) -> Result<(), MoveError> {
        if !(0..9).contains(&position) {
            return Err(MoveError::OutOfBound);
        };

        let field = &mut self.fields[position as usize];

        if field.is_some() {
            return Err(MoveError::NonEmptyField);
        }

        if let Some(index) = self
            .available_fields
            .iter()
            .position(|&value| value == position)
        {
            self.available_fields.swap_remove(index);
        }

        field.replace(player);

        Ok(())
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        // We collect the available fields in a HashSet because they are not guaranteed to be
        // ordered.
        self.fields == other.fields
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, field) in self.fields.iter().enumerate() {
            match field {
                Some(player) => write!(f, " {player} ")?,
                None => write!(f, "   ")?,
            };

            if i % 3 < 2 {
                write!(f, "|")?;
            } else {
                writeln!(f)?;

                if i == 8 {
                    break;
                }

                writeln!(f, "---+---+---")?;
            }
        }

        Ok(())
    }
}

impl Player {
    pub(crate) fn next_player(self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! state {
        ($($s:tt)+) => {
            State::from_array(fields![$($s)+])
        };
    }

    #[test]
    fn test_act() {
        let mut state = state![
            X O -
            - - -
            - - -
        ];

        assert!(Game::act(Player::X, 3, &mut state).is_ok());
        assert_eq!(
            state,
            state![
                X O -
                X - -
                - - -
            ]
        );
        assert_eq!(Game::available_moves(&state), &vec![2, 8, 4, 5, 6, 7]);

        assert!(Game::act(Player::X, 0, &mut state).is_err());

        assert!(Game::act(Player::O, 4, &mut state).is_ok());
        assert_eq!(
            state,
            state![
            X O -
            X O -
            - - -
            ]
        );
        assert_eq!(Game::available_moves(&state), &vec![2, 8, 7, 5, 6]);

        assert!(Game::act(Player::X, 8, &mut state).is_ok());
        assert_eq!(
            state,
            state![
            X O -
            X O -
            - - X
            ]
        );
        assert_eq!(Game::available_moves(&state), &vec![2, 6, 7, 5]);

        assert!(Game::act(Player::O, 7, &mut state).is_ok());
        assert_eq!(
            state,
            state![
            X O -
            X O -
            - O X
            ]
        );
        assert_eq!(Game::available_moves(&state), &vec![2, 6, 5]);
        assert_eq!(Game::status(&state), Status::Finished(Some(Player::O)));
    }

    #[test]
    fn test_status() {
        assert_eq!(
            Game::status(&state![
                X X X
                O O -
                - - -
            ]),
            Status::Finished(Some(Player::X))
        );
        assert_eq!(
            Game::status(&state![
                X - X
                O O -
                - - -
            ]),
            Status::OnGoing
        );
        assert_eq!(
            Game::status(&state![
                O X X
                O - -
                O X -
            ]),
            Status::Finished(Some(Player::O))
        );
        assert_eq!(
            Game::status(&state![
                O X O
                - X -
                O X -
            ]),
            Status::Finished(Some(Player::X))
        );
        assert_eq!(
            Game::status(&state![
                O X X
                O X -
                X O -
            ]),
            Status::Finished(Some(Player::X))
        );
        assert_eq!(
            Game::status(&state![
                X O X
                O O X
                - - -
            ]),
            Status::OnGoing
        );
        assert_eq!(
            Game::status(&state![
                X O X
                O X X
                O X O
            ]),
            Status::Finished(None)
        );
    }
}
