use std::fmt::Display;

pub trait Game: Display {
    fn play(&mut self) -> GameResult;

    fn is_terminal(&self) -> State;

    fn state(&self) -> &Board;
}

pub enum MoveError {
    NonEmptyField,
    OutOfBound,
}

pub struct Board {
    pub fields: [Option<Player>; 9],
    pub available_fields: Vec<usize>,
}
pub struct TicTacToe<'a, T, U>
where
    T: crate::Player,
    U: crate::Player,
{
    pub board: Board,
    current_player: Player,
    player_1: &'a T,
    player_2: &'a U,
}

pub enum State {
    Finished(Option<Player>),
    OnGoing,
}

pub enum Player {
    X,
    O,
}

pub enum GameResult {
    Victory,
    Draw,
    Loss,
}

impl<'a, T, U> TicTacToe<'a, T, U>
where
    T: crate::Player,
    U: crate::Player,
{
    pub fn new(player_1: &'a T, player_2: &'a U) -> Self {
        TicTacToe {
            board: Board::new(),
            current_player: Player::X,
            player_1,
            player_2,
        }
    }

    fn winner(&self) -> Option<Player> {
        match self.board.fields {
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

    fn act(&mut self, position: usize) -> Result<(), MoveError> {
        let next_player = self.current_player.next_player();
        let player = std::mem::replace(&mut self.current_player, next_player);

        self.board.act(player, position).map_err(|err| {
            self.current_player = self.current_player.next_player();
            err
        })
    }
}

impl<'a, T, U> Game for TicTacToe<'a, T, U>
where
    T: crate::Player,
    U: crate::Player,
{
    fn play(&mut self) -> GameResult {
        // This ensures that the players get randomly assigned as the first or second
        let first_player = fastrand::bool();

        let mut n: usize = 0;

        let winner = loop {
            let action = match n % 2 {
                0 => match first_player {
                    true => self.player_1.play(self, &self.board.available_fields),
                    false => self.player_2.play(self, &self.board.available_fields),
                },
                1 => match first_player {
                    true => self.player_2.play(self, &self.board.available_fields),
                    false => self.player_1.play(self, &self.board.available_fields),
                },
                _ => unreachable!(),
            };

            if self.act(action).is_ok() {
                n += 1;
            };

            if let State::Finished(winner) = self.is_terminal() {
                break winner;
            }
        };

        self.board.reset();

        match winner {
            Some(player) => match player {
                Player::X if first_player => GameResult::Victory,
                Player::X => GameResult::Loss,
                Player::O if !first_player => GameResult::Loss,
                Player::O => GameResult::Victory,
            },
            None => GameResult::Draw,
        }
    }

    fn is_terminal(&self) -> State {
        let winner = self.winner();

        if winner.is_some() {
            State::Finished(winner)
        } else if self.board.is_full() {
            State::Finished(None)
        } else {
            State::OnGoing
        }
    }

    fn state(&self) -> &Board {
        &self.board
    }
}

impl Board {
    pub fn new() -> Self {
        Board {
            fields: Default::default(),
            available_fields: vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
        }
    }

    pub fn is_full(&self) -> bool {
        self.fields.iter().all(|field| field.is_some())
    }

    pub fn reset(&mut self) {
        self.fields = Default::default();
        self.available_fields.clear();
        self.available_fields.extend(0..9);
    }

    fn act(&mut self, player: Player, position: usize) -> Result<(), MoveError> {
        if !(0..9).contains(&position) {
            return Err(MoveError::OutOfBound);
        };

        let field = &mut self.fields[position];

        if field.is_some() {
            return Err(MoveError::NonEmptyField);
        }

        // SAFETY: The original vector is ordered and we only remove elements, keeping it ordered.
        let idx = unsafe {
            self.available_fields
                .binary_search(&position)
                .unwrap_unchecked()
        };
        self.available_fields.remove(idx);

        field.replace(player);

        Ok(())
    }
}

impl<'a, T, U> Display for TicTacToe<'a, T, U>
where
    T: crate::Player,
    U: crate::Player,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.board)
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
