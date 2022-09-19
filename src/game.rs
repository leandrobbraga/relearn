use std::fmt::Display;

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

    fn is_terminal(&self, state: &Board) -> Status;

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
#[derive(Clone)]
pub struct Board {
    pub fields: [Option<Player>; 9],
    pub available_fields: Vec<usize>,
}
pub struct TicTacToe {}

pub enum Status {
    Finished(Option<Player>),
    OnGoing,
}

pub enum MoveError {
    NonEmptyField,
    OutOfBound,
}

#[derive(Clone)]
pub enum Player {
    X,
    O,
}

impl TicTacToe {
    pub fn new() -> Self {
        TicTacToe {}
    }

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

            if let Status::Finished(winner) = self.is_terminal(&board) {
                break winner;
            }
        }
    }

    fn is_terminal(&self, state: &Board) -> Status {
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
        if !(0..9).contains(&position) {
            return Err(MoveError::OutOfBound);
        };

        let field = &mut state.fields[position];

        if field.is_some() {
            return Err(MoveError::NonEmptyField);
        }

        if let Some(index) = state
            .available_fields
            .iter()
            .position(|value| *value == position)
        {
            state.available_fields.swap_remove(index);
        }

        field.replace(player);

        Ok(())
    }

    fn available_moves<'a>(&self, state: &'a Board) -> &'a Vec<usize> {
        &state.available_fields
    }
}

impl Board {
    pub fn new() -> Self {
        Board {
            fields: Default::default(),
            available_fields: vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
        }
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
    pub fn next_player(&self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}
