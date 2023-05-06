use serde::{Deserialize, Serialize};

use crate::game::{self, Game, State};

use super::Player;

#[derive(Serialize, Deserialize)]
pub struct RandomPlayer;

impl Player for RandomPlayer {
    fn play(&self, state: &State, _: game::Player) -> u8 {
        let available_moves = Game::available_moves(state);
        let i = fastrand::usize(..available_moves.len());
        available_moves[i]
    }

    fn learn(&mut self, _: &Game) {}

    fn save(&self) -> Result<(), super::ReLearnError> {
        Ok(())
    }
}
