use serde::{Deserialize, Serialize};

use super::Player;
use crate::{
    game::{self, Game, State},
    ReLearnError,
};
use std::io;

#[derive(Serialize, Deserialize)]
pub struct HumanPlayer;

impl Player for HumanPlayer {
    fn play(&self, state: &State, _: game::Player) -> u8 {
        let available_moves = Game::available_moves(state);

        println!("{state}");
        println!("Available moves: {available_moves:?}");

        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();

        let action: u8 = buf.trim().parse().unwrap();

        action
    }

    fn learn(&mut self, _: &Game) {}

    fn save(&self) -> Result<(), ReLearnError> {
        Ok(())
    }
}
