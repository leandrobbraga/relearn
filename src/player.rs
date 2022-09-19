use crate::game::Game;
use std::io;

pub struct HumanPlayer;
pub struct RandomPlayer;

pub trait Player {
    fn play(&self, game: &impl Game, available_moves: &[usize]) -> usize;
}

impl Player for HumanPlayer {
    fn play(&self, game: &impl Game, available_moves: &[usize]) -> usize {
        println!("{game}");
        println!("Available moves: {available_moves:?}");

        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();

        let action: usize = buf.trim().parse().unwrap();

        action
    }
}

impl Player for RandomPlayer {
    fn play(&self, _: &impl Game, available_moves: &[usize]) -> usize {
        let i = fastrand::usize(..available_moves.len());
        available_moves[i]
    }
}
