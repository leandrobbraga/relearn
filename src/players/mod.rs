mod human;
mod minmax;
mod random;

pub(crate) use human::HumanPlayer;
pub(crate) use minmax::MinMaxPlayer;
pub(crate) use random::RandomPlayer;

use crate::game::{self, Game, State};

pub trait Player {
    fn play(&self, game: &Game, state: &State, player: &game::Player) -> usize;
    fn learn(&mut self, game: &Game);
}
