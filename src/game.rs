use std::fmt::Display;

pub trait Game: Display {
    fn play(&mut self, player_1: &impl crate::Player, player_2: &impl crate::Player) -> GameResult;

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
pub struct TicTacToe {
    pub board: Board,
    current_player: Player,
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
    Player1,
    Draw,
    Player2,
}

impl TicTacToe {
    pub fn new() -> Self {
        TicTacToe {
            board: Board::new(),
            current_player: Player::X,
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

impl Game for TicTacToe {
    fn play(&mut self, player_1: &impl crate::Player, player_2: &impl crate::Player) -> GameResult {
        // This ensures that the players get randomly assigned as the first or second
        let mut first_player = true;

        let winner = loop {
            let action = match first_player {
                true => player_1.play(self, &self.board.available_fields),
                false => player_2.play(self, &self.board.available_fields),
            };

            if self.act(action).is_ok() {
                first_player = !first_player;
            };

            if let State::Finished(winner) = self.is_terminal() {
                break winner;
            }
        };

        self.board.reset();

        match winner {
            Some(player) => match player {
                Player::X => GameResult::Player1,
                Player::O => GameResult::Player2,
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

impl Display for TicTacToe {
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
