mod human;
pub mod minmax;
mod random;

pub(crate) use human::HumanPlayer;
pub(crate) use minmax::MinMaxPlayer;
pub(crate) use random::RandomPlayer;

use crate::{
    game::{self, Game, State},
    ReLearnError,
};

pub trait Player: Sync + Send {
    fn play(&self, state: &State, player: game::Player) -> u8;
    fn learn(&mut self, game: &Game);
    fn save(&self) -> Result<(), ReLearnError>;
}
