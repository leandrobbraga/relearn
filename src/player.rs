use crate::game::{Board, Game};
use std::io;

pub struct HumanPlayer;
pub struct RandomPlayer;

pub trait Player {
    fn play(&self, game: &impl Game, state: &Board) -> usize;
}

impl Player for HumanPlayer {
    fn play(&self, game: &impl Game, state: &Board) -> usize {
        let available_moves = game.available_moves(state);

        println!("{state}");
        println!("Available moves: {available_moves:?}");

        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();

        let action: usize = buf.trim().parse().unwrap();

        action
    }
}

impl Player for RandomPlayer {
    fn play(&self, game: &impl Game, board: &Board) -> usize {
        let available_moves = game.available_moves(board);
        let i = fastrand::usize(..available_moves.len());
        available_moves[i]
    }
}
