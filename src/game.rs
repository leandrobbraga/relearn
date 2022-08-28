use std::fmt::Display;

pub struct Game {
    pub board: Board,
    pub current_player: Player,
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Board {
                fields: Default::default(),
            },
            current_player: Player::X,
        }
    }

    pub fn winner(&self) -> Option<Player> {
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

    pub fn has_finished(&self) -> bool {
        if self.winner().is_some() {
            true
        } else {
            self.board.is_full()
        }
    }

    pub fn act(&mut self, position: usize) -> Result<(), MoveError> {
        if !(0..9).contains(&position) {
            return Err(MoveError::OutOfBound);
        };

        let field = &mut self.board.fields[position];

        if field.is_some() {
            return Err(MoveError::NonEmptyField);
        }

        field.replace(self.current_player.clone());
        self.current_player = self.current_player.next_player();

        Ok(())
    }

    pub fn available_actions(&self) -> Vec<usize> {
        self.board.available_fields()
    }
}

pub enum MoveError {
    NonEmptyField,
    OutOfBound,
}

pub struct Board {
    pub fields: [Option<Player>; 9],
}

impl Board {
    pub fn is_full(&self) -> bool {
        self.fields.iter().all(|field| field.is_some())
    }

    pub fn available_fields(&self) -> Vec<usize> {
        let mut available_fields = Vec::with_capacity(9);

        for (i, field) in self.fields.iter().enumerate() {
            if field.is_none() {
                available_fields.push(i);
            }
        }

        available_fields
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

#[derive(Clone, Debug)]
pub enum Player {
    X,
    O,
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
