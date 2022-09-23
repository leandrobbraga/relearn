use std::{collections::HashSet, fmt::Display};

/// A basic interface for any implemented game.
///
/// The trait is designed in a way that the game struct itself is stateless and the user needs to
/// pass the state around. This architecture make it easier to implement search algorithms where it
/// is necessary to branch the game.
///
/// For now, as we only have the `tic-tac-toe` implemented, we have a specialized state called
/// `Board`. In the future, when we implement more games, we might add a generic state interface.
pub trait Game {
    fn play(&self, player_1: &impl crate::Player, player_2: &impl crate::Player) -> Option<Player>;

    fn status(&self, state: &Board) -> Status;

    fn available_moves<'a>(&self, state: &'a Board) -> &'a Vec<usize>;

    /// Act in the state, mutating it.
    ///
    /// For now we'll keep this method as fallible for debugging purpose, we might implement a
    /// `unchecked_act` in the future for optimization purpose.
    fn act(&self, player: Player, action: usize, state: &mut Board) -> Result<(), MoveError>;
}

/// An specialized state for the `tic-tac-toe` game.
///
/// We implement the `Clone` trait for it to make possible for search algorithms to branch the
/// state.
#[derive(Clone, Debug, Eq)]
pub struct Board {
    fields: [Option<Player>; 9],
    available_fields: Vec<usize>,
}
pub struct TicTacToe {}

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Player {
    X,
    O,
}

impl TicTacToe {
    fn winner(&self, state: &Board) -> Option<Player> {
        match state.fields {
            [Some(Player::X), Some(Player::X), Some(Player::X), _, _, _, _, _, _]
            | [_, _, _, Some(Player::X), Some(Player::X), Some(Player::X), _, _, _]
            | [_, _, _, _, _, _, Some(Player::X), Some(Player::X), Some(Player::X)]
            | [Some(Player::X), _, _, Some(Player::X), _, _, Some(Player::X), _, _]
            | [_, Some(Player::X), _, _, Some(Player::X), _, _, Some(Player::X), _]
            | [_, _, Some(Player::X), _, _, Some(Player::X), _, _, Some(Player::X)]
            | [Some(Player::X), _, _, _, Some(Player::X), _, _, _, Some(Player::X)]
            | [_, _, Some(Player::X), _, Some(Player::X), _, Some(Player::X), _, _] => {
                Some(Player::X)
            }
            [Some(Player::O), Some(Player::O), Some(Player::O), _, _, _, _, _, _]
            | [_, _, _, Some(Player::O), Some(Player::O), Some(Player::O), _, _, _]
            | [_, _, _, _, _, _, Some(Player::O), Some(Player::O), Some(Player::O)]
            | [Some(Player::O), _, _, Some(Player::O), _, _, Some(Player::O), _, _]
            | [_, Some(Player::O), _, _, Some(Player::O), _, _, Some(Player::O), _]
            | [_, _, Some(Player::O), _, _, Some(Player::O), _, _, Some(Player::O)]
            | [Some(Player::O), _, _, _, Some(Player::O), _, _, _, Some(Player::O)]
            | [_, _, Some(Player::O), _, Some(Player::O), _, Some(Player::O), _, _] => {
                Some(Player::O)
            }
            _ => None,
        }
    }
}

impl Game for TicTacToe {
    fn play(&self, player_1: &impl crate::Player, player_2: &impl crate::Player) -> Option<Player> {
        let mut current_player = Player::X;
        let mut board = Board::new();

        loop {
            let action = match current_player {
                Player::X => player_1.play(self, &board),
                Player::O => player_2.play(self, &board),
            };

            let next_player = current_player.next_player();
            let player = std::mem::replace(&mut current_player, next_player);

            if self.act(player, action, &mut board).is_err() {
                // The same player tries again
                current_player = current_player.next_player();
                continue;
            };

            if let Status::Finished(winner) = self.status(&board) {
                break winner;
            }
        }
    }

    fn status(&self, state: &Board) -> Status {
        let winner = self.winner(state);

        if winner.is_some() {
            Status::Finished(winner)
        } else if state.available_fields.is_empty() {
            Status::Finished(None)
        } else {
            Status::OnGoing
        }
    }

    fn act(&self, player: Player, position: usize, state: &mut Board) -> Result<(), MoveError> {
        state.act(player, position)
    }

    fn available_moves<'a>(&self, state: &'a Board) -> &'a Vec<usize> {
        &state.available_fields
    }
}

impl Board {
    fn new() -> Self {
        Board {
            fields: Default::default(),
            available_fields: vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
        }
    }

    #[allow(unused)]
    fn from_array(fields: [Option<Player>; 9]) -> Self {
        let available_fields = fields
            .iter()
            .enumerate()
            .filter_map(|(idx, field)| if field.is_none() { Some(idx) } else { None })
            .collect();

        Self {
            fields,
            available_fields,
        }
    }

    fn act(&mut self, player: Player, position: usize) -> Result<(), MoveError> {
        if !(0..9).contains(&position) {
            return Err(MoveError::OutOfBound);
        };

        let field = &mut self.fields[position];

        if field.is_some() {
            return Err(MoveError::NonEmptyField);
        }

        if let Some(index) = self
            .available_fields
            .iter()
            .position(|value| *value == position)
        {
            self.available_fields.swap_remove(index);
        }

        field.replace(player);

        Ok(())
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        // We collect the available fields in a HashSet because they are not guaranteed to be
        // ordered.
        self.fields == other.fields
            && self.available_fields.iter().collect::<HashSet<_>>()
                == other.available_fields.iter().collect::<HashSet<_>>()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, field) in self.fields.iter().enumerate() {
            match field {
                Some(player) => write!(f, " {} ", player)?,
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

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

impl Player {
    fn next_player(&self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! state {
        (O) => {Some(Player::O)};
        (X) => {Some(Player::X)};
        (-) => {None};
        ($($s:tt)+) => {
            Board::from_array([$(state!($s)),+])
        };
    }

    #[test]
    fn test_act() {
        let game = TicTacToe {};

        let mut state = state![
            X O -
            - - -
            - - -
        ];

        assert!(game.act(Player::X, 3, &mut state).is_ok());

        assert_eq!(
            state,
            state![
                X O -
                X - -
                - - -
            ]
        );

        assert!(game.act(Player::X, 0, &mut state).is_err());
    }

    #[test]
    fn test_status() {
        let game = TicTacToe {};

        assert_eq!(
            game.status(&state![
                X X X
                O O -
                - - -
            ]),
            Status::Finished(Some(Player::X))
        );
        assert_eq!(
            game.status(&state![
                X - X
                O O -
                - - -
            ]),
            Status::OnGoing
        );
        assert_eq!(
            game.status(&state![
                O X X
                O - -
                O X -
            ]),
            Status::Finished(Some(Player::O))
        );
        assert_eq!(
            game.status(&state![
                O X O
                - X -
                O X -
            ]),
            Status::Finished(Some(Player::X))
        );
        assert_eq!(
            game.status(&state![
                O X X
                O X -
                X O -
            ]),
            Status::Finished(Some(Player::X))
        );
        assert_eq!(
            game.status(&state![
                X O X
                O O X
                - - -
            ]),
            Status::OnGoing
        );
        assert_eq!(
            game.status(&state![
                X O X
                O X X
                O X O
            ]),
            Status::Finished(None)
        );
    }

    #[test]
    fn test_available_moves() {
        let game = TicTacToe {};

        assert_eq!(
            game.available_moves(&state![
                X X X
                O O -
                - - -
            ]),
            &vec![5, 6, 7, 8]
        );
        assert_eq!(
            game.available_moves(&state![
                X - X
                O O -
                - - -
            ]),
            &vec![1, 5, 6, 7, 8]
        );
        assert_eq!(
            game.available_moves(&state![
                O X X
                O - -
                O X -
            ]),
            &vec![4, 5, 8]
        );
        assert_eq!(
            game.available_moves(&state![
                O X O
                - X -
                O X -
            ]),
            &vec![3, 5, 8]
        );
        assert_eq!(
            game.available_moves(&state![
                O X X
                O X -
                X O -
            ]),
            &vec![5, 8]
        );
        assert_eq!(
            game.available_moves(&state![
                X O X
                O O X
                - - -
            ]),
            &vec![6, 7, 8]
        );
        assert_eq!(
            game.available_moves(&state![
                X O X
                O X X
                O X O
            ]),
            &vec![]
        );
    }
}
